use super::super::*;

#[derive(Clone, PartialEq)]
pub enum Type {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class(String),
    Short,
    Boolean,
    Array(Box<Type>),
    Void,
    Unknown,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Type::Byte => "byte",
            Type::Char => "char",
            Type::Double => "double",
            Type::Float => "float",
            Type::Int => "int",
            Type::Long => "long",
            Type::Class(ty) => ty,
            Type::Short => "short",
            Type::Boolean => "boolean",
            Type::Array(ty) => {
                let ty: &Type = ty.as_ref();
                return f.write_str(format!("{}[]", ty).as_str());
            }
            Type::Void => "void",
            Type::Unknown => "unknown",
        })
    }
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        match value {
            4 => Type::Boolean,
            5 => Type::Char,
            6 => Type::Float,
            7 => Type::Double,
            8 => Type::Byte,
            9 => Type::Short,
            10 => Type::Int,
            11 => Type::Long,
            _ => panic!("Unknown value given: {}", value),
        }
    }
}

pub fn get_value(
    instructions_stack: &mut Vec<instruction::Instruction>,
    constant_pool: &mut ConstantPool,
    method_information: &super::MethodInformation,
) -> (String, Type) {
    let mut value = String::new();
    let ty: Type;

    println!("{:?}", instructions_stack);

    match instructions_stack
        .pop()
        .expect("No Instruction left in Instruction Stack")
    {
        instruction::Instruction::AALoad => {
            let index = get_value(instructions_stack, constant_pool, method_information).0;
            let aref = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("{aref}[{index}]", index = index, aref = aref.0).as_str());
            ty = get_array_type(aref.1);
        }

        instruction::Instruction::AConstNull => {
            value.push_str("null");
            ty = Type::Unknown;
        }

        instruction::Instruction::ALoad(variable_index) => {
            ty = Type::Unknown;

            if variable_index < method_information.arg_count {
                if !method_information.is_static && variable_index == 0 {
                    value.push_str("this");
                } else {
                    value.push_str(format!("arg{}", variable_index).as_str());
                }
            } else {
                value.push_str(format!("var{}", variable_index).as_str());
            }
        }

        instruction::Instruction::ALoad0 => {
            ty = Type::Unknown;

            if !method_information.is_static {
                value.push_str("this");
            } else if method_information.arg_count > 0 {
                value.push_str("arg0")
            } else {
                value.push_str("var0")
            }
        }

        instruction::Instruction::ALoad1 => {
            ty = Type::Unknown;

            if method_information.arg_count > 2 - method_information.is_static as u8 {
                value.push_str("arg1")
            } else {
                value.push_str("var1")
            }
        }

        instruction::Instruction::ALoad2 => {
            ty = Type::Unknown;

            if method_information.arg_count > 3 - method_information.is_static as u8 {
                value.push_str("arg2")
            } else {
                value.push_str("var2")
            }
        }

        instruction::Instruction::ALoad3 => {
            ty = Type::Unknown;

            if method_information.arg_count > 4 - method_information.is_static as u8 {
                value.push_str("arg3")
            } else {
                value.push_str("var3")
            }
        }

        instruction::Instruction::ANewArray(array_type_index) => {
            let index_count: u32 = get_value(instructions_stack, constant_pool, method_information)
                .0
                .parse()
                .unwrap();
            let array_type = format!(
                "L{}",
                class::get_class_name(array_type_index, constant_pool)
            );
            value.push_str(format!("new {}", value::get_type(array_type.clone())).as_str());
            value.push_str(format!("[{}]", index_count).as_str());
            ty = Type::Array(Box::new(Type::Class(value::get_type(array_type))));
        }

        instruction::Instruction::ArrayLength => {
            value.push_str(
                format!(
                    "{}.length",
                    get_value(instructions_stack, constant_pool, method_information).0
                )
                .as_str(),
            );
            ty = Type::Int;
        }

        instruction::Instruction::BALoad => {
            let index = get_value(instructions_stack, constant_pool, method_information);
            let arrayref = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{}[{}]", arrayref.0, index.0).as_str());

            ty = Type::Byte;
        }

        instruction::Instruction::BiPush(byte_value) => {
            value.push_str(byte_value.to_string().as_str());
            ty = Type::Byte;
        }

        instruction::Instruction::CALoad => {
            let index = get_value(instructions_stack, constant_pool, method_information);
            let arrayref = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{}[{}]", arrayref.0, index.0).as_str());

            ty = Type::Char;
        }

        instruction::Instruction::D2F => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(float) {}", val.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::D2I => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(int) {}", val.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::D2L => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(long) {}", val.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::DAdd => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} + {}", value1.0, value2.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::DALoad => {
            let index = get_value(instructions_stack, constant_pool, method_information);
            let arrayref = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{}[{}]", arrayref.0, index.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::DCmpG => {
            let value1 = get_value(instructions_stack, constant_pool, method_information);
            let value2 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("DCmpG ({}, {})", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::DCmpL => {
            let value1 = get_value(instructions_stack, constant_pool, method_information);
            let value2 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("DCmpL ({}, {})", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::DConst0 => {
            value.push_str("0.0");
            ty = Type::Double
        }

        instruction::Instruction::DConst1 => {
            value.push_str("1.0");
            ty = Type::Double
        }

        instruction::Instruction::DDiv => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} / {}", value1.0, value2.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::DLoad(variable_index) => {
            ty = Type::Double;

            if variable_index < method_information.arg_count {
                if !method_information.is_static && variable_index == 0 {
                    value.push_str("this");
                } else {
                    value.push_str(format!("arg{}", variable_index).as_str());
                }
            } else {
                value.push_str(format!("var{}", variable_index).as_str());
            }
        }

        instruction::Instruction::DLoad0 => {
            ty = Type::Double;

            if !method_information.is_static {
                value.push_str("this");
            } else if method_information.arg_count > 0 {
                value.push_str("arg0")
            } else {
                value.push_str("var0")
            }
        }

        instruction::Instruction::DLoad1 => {
            ty = Type::Double;

            if method_information.arg_count > 2 - method_information.is_static as u8 {
                value.push_str("arg1")
            } else {
                value.push_str("var1")
            }
        }

        instruction::Instruction::DLoad2 => {
            ty = Type::Double;

            if method_information.arg_count > 3 - method_information.is_static as u8 {
                value.push_str("arg2")
            } else {
                value.push_str("var2")
            }
        }

        instruction::Instruction::DLoad3 => {
            ty = Type::Double;

            if method_information.arg_count > 4 - method_information.is_static as u8 {
                value.push_str("arg3")
            } else {
                value.push_str("var3")
            }
        }

        instruction::Instruction::DMul => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} * {}", value1.0, value2.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::DNeg => {
            let val = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("-{}", val.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::DRem => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} % {}", value1.0, value2.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::DSub => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} - {}", value1.0, value2.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::F2D => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(float) {}", val.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::F2I => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(int) {}", val.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::F2L => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(long) {}", val.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::FAdd => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} + {}", value1.0, value2.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::FALoad => {
            let index = get_value(instructions_stack, constant_pool, method_information);
            let arrayref = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{}[{}]", arrayref.0, index.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::FConst0 => {
            value.push_str("0.0f");
            ty = Type::Float;
        }

        instruction::Instruction::FConst1 => {
            value.push_str("1.0f");
            ty = Type::Float;
        }

        instruction::Instruction::FConst2 => {
            value.push_str("2.0f");
            ty = Type::Float;
        }

        instruction::Instruction::FDiv => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} / {}", value1.0, value2.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::FLoad(variable_index) => {
            if variable_index < method_information.arg_count {
                if !method_information.is_static && variable_index == 0 {
                    value.push_str("this");
                } else {
                    value.push_str(format!("arg{}", variable_index).as_str());
                }
            } else {
                value.push_str(format!("var{}", variable_index).as_str());
            }

            ty = Type::Float;
        }

        instruction::Instruction::FLoad0 => {
            ty = Type::Float;

            if !method_information.is_static {
                value.push_str("this");
            } else if method_information.arg_count > 0 {
                value.push_str("arg0")
            } else {
                value.push_str("var0")
            }
        }

        instruction::Instruction::FLoad1 => {
            ty = Type::Float;

            if method_information.arg_count > 2 - method_information.is_static as u8 {
                value.push_str("arg1")
            } else {
                value.push_str("var1")
            }
        }

        instruction::Instruction::FLoad2 => {
            ty = Type::Float;

            if method_information.arg_count > 3 - method_information.is_static as u8 {
                value.push_str("arg2")
            } else {
                value.push_str("var2")
            }
        }

        instruction::Instruction::FLoad3 => {
            ty = Type::Float;

            if method_information.arg_count > 4 - method_information.is_static as u8 {
                value.push_str("arg3")
            } else {
                value.push_str("var3")
            }
        }

        instruction::Instruction::FMul => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} * {}", value1.0, value2.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::FNeg => {
            let val = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("-{}", val.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::FRem => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} % {}", value1.0, value2.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::FSub => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} - {}", value1.0, value2.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::GetField(index) => {
            let objectref = get_value(instructions_stack, constant_pool, method_information);

            let field = match constant_pool.get_index(index) {
                CPIndexType::FieldRef {
                    class_index,
                    name_and_type_index,
                } => (class_index, name_and_type_index),
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let class_name = class::get_class_name(field.0, constant_pool);

            let name_and_type = match constant_pool.get_index(field.1) {
                CPIndexType::NameAndType {
                    name_index,
                    descriptor_index,
                } => (name_index, descriptor_index),
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let name = match constant_pool.get_index(name_and_type.0) {
                CPIndexType::Utf8(string) => string,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let field_type = match constant_pool.get_index(name_and_type.0) {
                CPIndexType::Utf8(string) => string,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            ty = Type::Class(value::get_type(format!("L{}", field_type)));

            value.push_str(format!("{}.{}", objectref.0, name).as_str());
        }

        instruction::Instruction::GetStatic(index) => {
            let field = match constant_pool.get_index(index) {
                CPIndexType::FieldRef {
                    class_index,
                    name_and_type_index,
                } => (class_index, name_and_type_index),
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let class_name = class::get_class_name(field.0, constant_pool);

            let name_and_type = match constant_pool.get_index(field.1) {
                CPIndexType::NameAndType {
                    name_index,
                    descriptor_index,
                } => (name_index, descriptor_index),
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let name = match constant_pool.get_index(name_and_type.0) {
                CPIndexType::Utf8(string) => string,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let field_type = match constant_pool.get_index(name_and_type.0) {
                CPIndexType::Utf8(string) => string,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            ty = Type::Class(value::get_type(format!("L{}", field_type)));

            value.push_str(format!("{}.{}", class_name, name).as_str());
        }

        instruction::Instruction::I2B => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(float) {}", val.0).as_str());

            ty = Type::Byte;
        }

        instruction::Instruction::I2C => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(int) {}", val.0).as_str());

            ty = Type::Char;
        }

        instruction::Instruction::I2D => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(long) {}", val.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::I2F => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(long) {}", val.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::I2L => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(long) {}", val.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::I2S => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(long) {}", val.0).as_str());

            ty = Type::Short;
        }

        instruction::Instruction::IAdd => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} + {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IALoad => {
            let index = get_value(instructions_stack, constant_pool, method_information);
            let arrayref = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{}[{}]", arrayref.0, index.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IAnd => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} & {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IConstM1 => {
            value.push('1');
            ty = Type::Int;
        }

        instruction::Instruction::IConst0 => {
            value.push('0');
            ty = Type::Int;
        }

        instruction::Instruction::IConst1 => {
            value.push('1');
            ty = Type::Int;
        }

        instruction::Instruction::IConst2 => {
            value.push('2');
            ty = Type::Int;
        }

        instruction::Instruction::IConst3 => {
            value.push('3');
            ty = Type::Int;
        }

        instruction::Instruction::IConst4 => {
            value.push('4');
            ty = Type::Int;
        }

        instruction::Instruction::IConst5 => {
            value.push('5');
            ty = Type::Int;
        }

        instruction::Instruction::IDiv => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} / {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IInc(index, con) => {
            if index < method_information.arg_count {
                if !method_information.is_static && index == 0 {
                    value.push_str("this");
                } else {
                    value.push_str(format!("arg{}", index).as_str());
                }
            } else {
                value.push_str(format!("var{}", index).as_str());
            }

            value.push_str(format!(" + {}", con).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::ILoad(variable_index) => {
            if variable_index < method_information.arg_count {
                if !method_information.is_static && variable_index == 0 {
                    value.push_str("this");
                } else {
                    value.push_str(format!("arg{}", variable_index).as_str());
                }
            } else {
                value.push_str(format!("var{}", variable_index).as_str());
            }

            ty = Type::Int;
        }

        instruction::Instruction::ILoad0 => {
            ty = Type::Int;

            if !method_information.is_static {
                value.push_str("this");
            } else if method_information.arg_count > 0 {
                value.push_str("arg0")
            } else {
                value.push_str("var0")
            }
        }

        instruction::Instruction::ILoad1 => {
            ty = Type::Int;

            if method_information.arg_count > 2 - method_information.is_static as u8 {
                value.push_str("arg1")
            } else {
                value.push_str("var1")
            }
        }

        instruction::Instruction::ILoad2 => {
            ty = Type::Int;

            if method_information.arg_count > 3 - method_information.is_static as u8 {
                value.push_str("arg2")
            } else {
                value.push_str("var2")
            }
        }

        instruction::Instruction::ILoad3 => {
            ty = Type::Int;

            if method_information.arg_count > 4 - method_information.is_static as u8 {
                value.push_str("arg3")
            } else {
                value.push_str("var3")
            }
        }

        instruction::Instruction::IMul => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} * {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::INeg => {
            let val = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("-{}", val.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::InstanceOf(index) => {
            let class_name = class::get_class_name(index, constant_pool);
            let objectref = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} instanceof {}", objectref.0, class_name).as_str());

            ty = Type::Boolean;
        }

        // Static overridden method
        instruction::Instruction::InvokeDynamic(index, _, _) => {
            let methodref = match constant_pool.get_index(index) {
                CPIndexType::MethodRef {
                    class_index,
                    name_and_type_index,
                } => (class_index, name_and_type_index),
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let class_name = class::get_class_name(methodref.0, constant_pool);

            let name_and_type = match constant_pool.get_index(methodref.1) {
                CPIndexType::NameAndType {
                    name_index,
                    descriptor_index,
                } => (name_index, descriptor_index),
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let name = match constant_pool.get_index(name_and_type.0) {
                CPIndexType::Utf8(name) => name,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let descriptor = match constant_pool.get_index(name_and_type.1) {
                CPIndexType::Utf8(name) => name,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            value.push_str(format!("{}.{}", class_name, name).as_str());

            ty = Type::Unknown;
        }

        instruction::Instruction::InvokeInterface(index, _, _) => {
            let methodref = match constant_pool.get_index(index) {
                CPIndexType::MethodRef {
                    class_index,
                    name_and_type_index,
                } => (class_index, name_and_type_index),
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let class_name = class::get_class_name(methodref.0, constant_pool);

            let name_and_type = match constant_pool.get_index(methodref.1) {
                CPIndexType::NameAndType {
                    name_index,
                    descriptor_index,
                } => (name_index, descriptor_index),
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let name = match constant_pool.get_index(name_and_type.0) {
                CPIndexType::Utf8(name) => name,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            let descriptor = match constant_pool.get_index(name_and_type.1) {
                CPIndexType::Utf8(name) => name,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            value.push_str(format!("{}.{}", class_name, name).as_str());

            ty = Type::Unknown;
        }

        instruction::Instruction::IOr => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} | {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IRem => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} % {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IShl => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} << {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IShr => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} >> {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::ISub => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} - {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IUShr => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} >>> {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::IXor => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} ^ {}", value1.0, value2.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::L2D => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(float) {}", val.0).as_str());

            ty = Type::Double;
        }

        instruction::Instruction::L2F => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(int) {}", val.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::L2I => {
            let val = get_value(instructions_stack, constant_pool, method_information);
            value.push_str(format!("(long) {}", val.0).as_str());

            ty = Type::Int;
        }

        instruction::Instruction::LAdd => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} + {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LALoad => {
            let index = get_value(instructions_stack, constant_pool, method_information);
            let arrayref = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{}[{}]", arrayref.0, index.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LAnd => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} & {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LCmp => {
            todo!("LCmp");
        }

        instruction::Instruction::LConst0 => {
            value.push_str("0L");
            ty = Type::Long;
        }

        instruction::Instruction::LConst1 => {
            value.push_str("1L");
            ty = Type::Long;
        }

        instruction::Instruction::LDiv => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} / {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LLoad(variable_index) => {
            if variable_index < method_information.arg_count {
                if !method_information.is_static && variable_index == 0 {
                    value.push_str("this");
                } else {
                    value.push_str(format!("arg{}", variable_index).as_str());
                }
            } else {
                value.push_str(format!("var{}", variable_index).as_str());
            }

            ty = Type::Long;
        }

        instruction::Instruction::LLoad0 => {
            ty = Type::Long;

            if !method_information.is_static {
                value.push_str("this");
            } else if method_information.arg_count > 0 {
                value.push_str("arg0")
            } else {
                value.push_str("var0")
            }
        }

        instruction::Instruction::LLoad1 => {
            ty = Type::Long;

            if method_information.arg_count > 2 - method_information.is_static as u8 {
                value.push_str("arg1")
            } else {
                value.push_str("var1")
            }
        }

        instruction::Instruction::LLoad2 => {
            ty = Type::Long;

            if method_information.arg_count > 3 - method_information.is_static as u8 {
                value.push_str("arg2")
            } else {
                value.push_str("var2")
            }
        }

        instruction::Instruction::LLoad3 => {
            ty = Type::Long;

            if method_information.arg_count > 4 - method_information.is_static as u8 {
                value.push_str("arg3")
            } else {
                value.push_str("var3")
            }
        }

        instruction::Instruction::LMul => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} * {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LNeg => {
            let val = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("-{}", val.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LOr => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} | {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LRem => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} % {}", value1.0, value2.0).as_str());

            ty = Type::Float;
        }

        instruction::Instruction::LShl => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} << {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LShr => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} >> {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LSub => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} - {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LUShr => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} >>> {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::LXor => {
            let value2 = get_value(instructions_stack, constant_pool, method_information);
            let value1 = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{} ^ {}", value1.0, value2.0).as_str());

            ty = Type::Long;
        }

        instruction::Instruction::Ldc(cp_index) => {
            let index = get_cp_value(cp_index as u16, constant_pool);

            value.push_str(index.0.as_str());

            ty = index.1;
        }

        instruction::Instruction::LdcW(cp_index) => {
            let index = get_cp_value(cp_index as u16, constant_pool);

            value.push_str(index.0.as_str());

            ty = index.1;
        }

        instruction::Instruction::Ldc2W(index) => {
            let index = constant_pool.get_index(index);
            let index = match index {
                CPIndexType::Double(value) => {
                    ty = Type::Double;
                    
                    value.to_string()
                }

                CPIndexType::Integer(value) => {
                    ty = Type::Float;
                    value.to_string()
                }
                
                _ => panic!("Invalid Type in Constant Pool: {:?}", index),
            };

            value.push_str(index.as_str());
        }

        instruction::Instruction::MultiANewArray(index, dimensions) => {
            let mut class_name = value::get_type(class::get_class_name(index, constant_pool));

            ty = Type::Class(class_name.clone());

            let mut sizes = Vec::new();

            for _i in 0..dimensions {
                sizes.push(
                    get_value(instructions_stack, constant_pool, method_information)
                        .0
                        .parse::<i32>()
                        .unwrap(),
                );
            }
            sizes.reverse();

            for i in 0..dimensions {
                class_name =
                    class_name.replacen("[]", format!("[{}]", sizes[i as usize]).as_str(), 1);
            }
            value.push_str(format!("new {}", class_name).as_str());
        }

        instruction::Instruction::New(index) => {
            let class_name = class::get_class_name(index, constant_pool);

            value.push_str(format!("new {}()", class_name).as_str());

            ty = Type::Class(class_name);
        }

        instruction::Instruction::NewArray(atype) => {
            ty = Type::from(atype);

            let count = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("new {}[{}]", ty, count.0).as_str());
        }

        instruction::Instruction::SALoad => {
            let index = get_value(instructions_stack, constant_pool, method_information);
            let arrayref = get_value(instructions_stack, constant_pool, method_information);

            value.push_str(format!("{}[{}]", arrayref.0, index.0).as_str());

            ty = Type::Short;
        }

        instruction::Instruction::SiPush(short_value) => {
            value.push_str(short_value.to_string().as_str());
            ty = Type::Short;
        }

        instruction => {
            // ty = Type::Unknown;
            unimplemented!("{:?}", instruction)
        }
    };
    (value, ty)
}

fn get_array_type(ty: Type) -> Type {
    let aref;
    match ty {
        Type::Array(box_type) => aref = box_type,
        _ => return ty,
    }
    let mut iter_ty: Type = aref.as_ref().clone();
    while match iter_ty {
        Type::Array(_) => true,
        _ => return iter_ty,
    } {
        let aref = match iter_ty {
            Type::Array(ty) => ty,
            _ => return iter_ty,
        };
        iter_ty = aref.as_ref().clone();
    }
    panic!()
}

fn get_cp_value(index: u16, constant_pool: &mut ConstantPool) -> (String, Type) {
    let ty: Type;

    let index = constant_pool.get_index(index);
    let index = match index {
        CPIndexType::Class(name_index) => {
            let class_name = match constant_pool.get_index(name_index) {
                CPIndexType::Utf8(a) => a,
                _ => panic!("Invalid Type in Constant Pool"),
            };

            ty = Type::Class(class_name.clone());

            class_name
        }
        CPIndexType::Float(value) => {
            ty = Type::Float;
            value.to_string()
        }
        CPIndexType::Integer(value) => {
            ty = Type::Int;
            value.to_string()
        }
        CPIndexType::String(index) => {
            ty = Type::Class("java.lang.String".to_string());
            let index = constant_pool.get_index(index);
            let value = match index {
                CPIndexType::Utf8(string) => string,
                _ => panic!("Invalid Type in Constant Pool"),
            };
            let mut string = String::new();
            string.push('"');
            string.push_str(value.as_str());
            string.push('"');
            string
        }
        _ => panic!("Invalid Type in Constant Pool: {:?}", index),
    };

    (index, ty)
}