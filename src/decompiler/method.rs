use super::attribute;
use std::io::{BufReader, Read};

const PUBLIC: u16 = 0x0001;
const PRIVATE: u16 = 0x0002;
const PROTECTED: u16 = 0x0004;
const STATIC: u16 = 0x0008;
const FINAL: u16 = 0x0010;
const SYNCHRONIZED: u16 = 0x0020;
const BRIDGE: u16 = 0x0040;
const VARARGS: u16 = 0x0080;
const NATIVE: u16 = 0x0100;
const ABSTRACT: u16 = 0x0400;
const STRICT: u16 = 0x0800;
const SYNTHETIC: u16 = 0x1000;

pub fn decompile_methods(class_file: &mut super::ClassFile) -> String {
    let method_count = class_file.methods.method_count;
    let methods = &class_file.methods.methods;

    let mut methods_string = String::new();

    for i in 0..method_count {
        let method_info = &methods[i as usize];

        let mut method = Method::new(
            class_file.this_class,
            method_info,
            &mut class_file.constant_pool,
        );

        methods_string.push_str(method.decompile_method().as_str());
    }

    methods_string
}

struct Method<'a> {
    decompile: bool,
    varargs: bool,
    is_static: bool,
    arg_count: u8,
    this_class: u16,
    method_info: &'a super::MethodInfo,
    method_signature: String,
    constant_pool: &'a mut super::ConstantPool,
}

impl<'a> Method<'a> {
    pub fn new(
        this_class: u16,
        method_info: &'a super::MethodInfo,
        constant_pool: &'a mut super::ConstantPool,
    ) -> Self {
        Method {
            decompile: true,
            varargs: false,
            is_static: false,
            arg_count: 0,
            this_class,
            method_info,
            method_signature: String::new(),
            constant_pool,
        }
    }

    pub fn decompile_method(&mut self) -> String {
        let mut method_code = String::new();

        method_code.push('\t');
        method_code.push_str(self.get_method_signature());

        if !self.decompile {
            method_code.push_str(";\n");
            return method_code;
        }

        method_code.push_str(" {\n");

        let code = self.decompile_bytecode();

        let code = code.split('\n');
        for string in code {
            method_code.push_str("\t\t");
            method_code.push_str(string);
            method_code.push('\n');
        }

        method_code.push_str("\t}");

        method_code.push('\n');
        method_code
    }

    fn get_method_signature(&mut self) -> &String {
        if self.method_signature.is_empty() {
            self.generate_method_signature();
        }

        &self.method_signature
    }

    fn generate_method_signature(&mut self) {
        let mut method_signature = String::new();

        let access_flags = self.method_info.access_flags;

        if access_flags & PUBLIC != 0 {
            method_signature.push_str("public ");
        } else if access_flags & PRIVATE != 0 {
            method_signature.push_str("private ");
        } else if access_flags & PROTECTED != 0 {
            method_signature.push_str("protected ");
        }

        if access_flags & SYNTHETIC != 0 {
            method_signature.push_str("/* synthetic */ ");
        }

        if access_flags & STATIC != 0 {
            method_signature.push_str("static ");
            self.is_static = true;
        }

        if access_flags & ABSTRACT != 0 {
            method_signature.push_str("abstract ");
            self.decompile = false;
        }

        if access_flags & FINAL != 0 {
            method_signature.push_str("final ");
        }

        if access_flags & SYNCHRONIZED != 0 {
            method_signature.push_str("synchronized ");
        }

        if access_flags & BRIDGE != 0 {
            method_signature.push_str("/* brigde */ ");
        }

        if access_flags & VARARGS != 0 {
            self.varargs = true;
        }

        if access_flags & STRICT != 0 {
            method_signature.push_str("/* strict */ ");
        }

        if access_flags & NATIVE != 0 {
            method_signature.push_str("native ");
            self.decompile = false;
        }

        let descriptor = match self
            .constant_pool
            .get_index(self.method_info.descriptor_index)
        {
            super::CPIndexType::Utf8(return_type) => return_type,
            _ => panic!("Invalid Type in Constant Pool"),
        };

        let (args, return_value) =
            descriptor.split_at(descriptor.find(")").expect("Invalid descriptor String") + 1);

        method_signature
            .push_str(format!("{} ", super::value::get_type(return_value.to_string())).as_str());
        method_signature.push_str(self.get_string(self.method_info.name_index).as_str());

        method_signature = method_signature.replace(
            "void <init>",
            super::class::get_class_name(self.this_class, self.constant_pool)
                .split('/')
                .last()
                .unwrap(),
        );

        let mut args = args.to_string();
        args.retain(|a| a != '(' && a != ')');

        let mut fmt_args = String::new();

        let mut arg_offset = 1;
        let mut index = 0;

        if self.is_static {
            arg_offset = 0;
        }

        for string in self.parse_args(args) {
            if !string.is_empty() {
                if index > 0 {
                    fmt_args.push_str(", ");
                }

                fmt_args.push_str(format!("{} arg{}", string, index + arg_offset).as_str());
                index += 1;
            }
        }

        self.arg_count = index + arg_offset;

        let mut args = String::new();

        if self.varargs {
            let last_array_index = fmt_args
                .rfind("[]")
                .expect("Invalid arguments: Method with varargs must contain an Array")
                .clone();

            let (first_args, last_arg) = fmt_args.split_at(last_array_index);

            let last_arg = last_arg.replace("[]", "...");

            args.push_str(first_args);
            args.push_str(last_arg.as_str());
        } else {
            args = fmt_args;
        }

        method_signature.push_str(format!("({})", args).as_str());

        method_signature = method_signature.replace("static void <clinit>()", "static");

        let mut exceptions: Option<&Vec<u8>> = None;

        for attribute in &self.method_info.attributes {
            if exceptions == None {
                let attribute_name = match &attribute.attribute_name_index.value {
                    super::CPIndexType::Utf8(name) => name,
                    _ => panic!("Invalid Type in Constant Pool"),
                };

                exceptions = match attribute_name.as_str() {
                    "Exceptions" => Some(&attribute.info),
                    _ => None,
                };
            }
        }

        match exceptions {
            Some(exceptions) => {
                method_signature.push_str(" throws ");
                let exception_count: [u8; 2] = [exceptions[0], exceptions[1]];
                let exception_count = u16::from_be_bytes(exception_count);

                for i in 0..exception_count as usize {
                    let mut class_name = String::new();

                    if i > 0 {
                        method_signature.push_str(", ");
                    }

                    let exception_index =
                        u16::from_be_bytes([exceptions[2 * (i + 1)], exceptions[2 * (i + 1) + 1]]);

                    class_name.push('L');
                    class_name.push_str(
                        super::class::get_class_name(exception_index, self.constant_pool).as_str(),
                    );

                    method_signature.push_str(super::value::get_type(class_name).as_str());
                }
            }
            None => {}
        };

        self.method_signature = method_signature;
    }

    fn parse_args(&self, args: String) -> Vec<String> {
        let mut types = Vec::<String>::new();

        let mut chars = args.chars();

        while chars.clone().count() != 0 {
            let char = chars.next().unwrap();

            match char {
                'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'V' | 'Z' => {
                    types.push(super::value::get_type(char.to_string()));
                }
                'L' => {
                    let mut type_string = String::new();
                    let mut char = chars.next().unwrap();
                    while char != ';' {
                        type_string.push(char);
                        if chars.clone().count() == 0 {
                            break;
                        }
                        char = chars.next().unwrap();
                    }
                    types.push(type_string.replace('/', "."));
                }
                '[' => {
                    let mut type_string = String::new();
                    let mut char = chars.next().unwrap();
                    while char != ';' {
                        type_string.push(char);
                        if chars.clone().count() == 0 {
                            break;
                        }
                        char = chars.next().unwrap();
                    }
                    let type_string = self.parse_args(type_string);
                    let mut type_string = type_string.get(0).unwrap().to_string();
                    type_string.push_str("[]");
                    types.push(type_string);
                }
                type_string => panic!("Invalid Type: {}", type_string),
            }
        }
        types
    }

    fn decompile_bytecode(&mut self) -> String {
        let mut bytecode: Option<&Vec<u8>> = None;

        for attribute in &self.method_info.attributes {
            if bytecode == None {
                let attribute_name = match &attribute.attribute_name_index.value {
                    super::CPIndexType::Utf8(name) => name,
                    _ => panic!("Invalid Type in Constant Pool"),
                };

                bytecode = match attribute_name.as_str() {
                    "Code" => Some(&attribute.info),
                    _ => None,
                };
            }
        }

        let mut bytecode = match bytecode {
            Some(bytecode) => bytecode,
            None => panic!("No Method Bytecode found"),
        }
        .take(100);

        let mut _max_stack = [0u8; 2];
        let mut _max_locales = [0u8; 2];

        bytecode
            .read_exact(&mut _max_stack)
            .expect("Invalid Method Bytecode");
        bytecode
            .read_exact(&mut _max_locales)
            .expect("Invalid Method Bytecode");

        let code_length_bytes: &mut [u8; 4] = &mut [0; 4];
        bytecode
            .read_exact(code_length_bytes)
            .expect("Invalid Method Bytecode");
        let code_length = u32::from_be_bytes(*code_length_bytes) as usize;

        bytecode.set_limit(code_length as u64);

        let mut code_stream = Vec::<u8>::with_capacity(code_length);

        for _i in 0..code_length {
            code_stream.push(0);
        }

        bytecode
            .read_exact(code_stream.as_mut_slice())
            .expect("Invalid Method Bytecode");

        let code_stream = BufReader::with_capacity(code_length, code_stream.as_slice());

        bytecode.set_limit(2);

        let exception_table_length_bytes: &mut [u8; 2] = &mut [0; 2];

        bytecode
            .read_exact(exception_table_length_bytes)
            .expect("Invalid Method Bytecode");

        let exception_table_length = u16::from_be_bytes(*exception_table_length_bytes);

        let mut exception_table_bytes = Vec::<u8>::with_capacity(code_length);

        for _i in 0..exception_table_length * 8 {
            exception_table_bytes.push(0);
        }

        bytecode.set_limit(exception_table_length as u64 * 8);

        bytecode
            .read_exact(exception_table_bytes.as_mut_slice())
            .expect("Invalid Method Bytecode");

        bytecode.set_limit(2);

        let attributes_count_bytes: &mut [u8; 2] = &mut [0; 2];

        bytecode
            .read_exact(attributes_count_bytes)
            .expect("Invalid Method Bytecode");

        let attributes_count = u16::from_be_bytes(*attributes_count_bytes);

        let mut attributes: Vec<attribute::Attribute> = Vec::new();

        // let mut variable_table: Option<super::variable::VariableTable> = None;


        for _ in 0..attributes_count {
            bytecode.set_limit(6);

            let attribute_name_index_bytes: &mut [u8; 2] = &mut [0; 2];

            bytecode
                .read_exact(attribute_name_index_bytes)
                .expect("Invalid Method Bytecode");

            let attribute_length_bytes: &mut [u8; 4] = &mut [0; 4];

            bytecode
                .read_exact(attribute_length_bytes)
                .expect("Invalid Method Bytecode");

            let attribute_length = u32::from_be_bytes(*attribute_length_bytes);

            bytecode.set_limit(attribute_length as u64);

            let mut attribute_bytes: Vec<u8> = Vec::new();

            for _i in 0..attribute_length {
                attribute_bytes.push(0);
            }

            bytecode
                .read_exact(attribute_bytes.as_mut_slice())
                .expect("Invalid Method Bytecode");
            let name_index = u16::from_be_bytes(*attribute_name_index_bytes);
            let attribute =
                attribute::Attribute::new(name_index, attribute_length, attribute_bytes);

            if self.get_string(attribute.get_name_index()) == "LocalVariableTable" {
                bytecode.set_limit(2);

                let length_bytes: &mut [u8; 2] = &mut [0, 0];

                bytecode
                    .read_exact(length_bytes)
                    .expect("Invalid Method Bytecode");

                let length = u16::from_be_bytes(*length_bytes);

                // let mut variables: Vec<super::codegen::variable::Variable> = Vec::new();

                for _i in 0..length {
                    let start_pc_bytes: &mut [u8; 2] = &mut [0, 0];

                    bytecode
                        .read_exact(start_pc_bytes)
                        .expect("Invalid Method Bytecode");

                    let start_pc = u16::from_be_bytes(*start_pc_bytes);

                    let length_bytes: &mut [u8; 2] = &mut [0, 0];

                    bytecode
                        .read_exact(length_bytes)
                        .expect("Invalid Method Bytecode");

                    let length = u16::from_be_bytes(*length_bytes);

                    let name_index_bytes: &mut [u8; 2] = &mut [0, 0];

                    bytecode
                        .read_exact(name_index_bytes)
                        .expect("Invalid Method Bytecode");

                    let name_index = u16::from_be_bytes(*name_index_bytes);

                    let descriptor_index_bytes: &mut [u8; 2] = &mut [0, 0];

                    bytecode
                        .read_exact(descriptor_index_bytes)
                        .expect("Invalid Method Bytecode");

                    let descriptor_index = u16::from_be_bytes(*descriptor_index_bytes);

                    let index_bytes: &mut [u8; 2] = &mut [0, 0];

                    bytecode
                        .read_exact(index_bytes)
                        .expect("Invalid Method Bytecode");

                    let index = u16::from_be_bytes(*index_bytes);

                    // let variable = super::variable::Variable {
                    //     start_pc,
                    //     length,
                    //     name_index,
                    //     descriptor_index,
                    //     index,
                    // };
                    // variables.push(variable);
                }
                // variable_table = Some(super::variable::VariableTable {
                //     variable_count: length,
                //     variables: variables.clone(),
                // })
            }

            attributes.push(attribute);
        }

        // println!("{:#?}", attributes);

        let instructions = super::instruction::parse(code_stream, code_length);
        // println!("{:?}", instructions);
        super::codegen::generate_code(instructions, self.constant_pool, &super::codegen::MethodInformation {
            is_static: self.is_static,
            arg_count: self.arg_count
        })

        // TODO: Local Variables
    }

    fn get_string(&mut self, index: u16) -> String {
        let method_name = self.constant_pool.get_index(index);

        match method_name {
            super::CPIndexType::Utf8(a) => a,
            _ => panic!("Invalid Type in Constant Pool"),
        }
    }
}
