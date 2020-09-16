const PUBLIC: u16 = 0x0001;
const PRIVATE: u16 = 0x0002;
const PROTECTED: u16 = 0x0004;
const STATIC: u16 = 0x0008;
const FINAL: u16 = 0x0010;
const VOLATILE: u16 = 0x0040;
const TRANSIENT: u16 = 0x0080;
const SYNTHETIC: u16 = 0x1000;
// idk how to use it but it doesn't look really important
// const ENUM: u16 = 0x4000;

pub fn decompile_fields(class_file: &mut super::ClassFile) -> String {
    let field_count = class_file.fields.field_count;
    let fields = &class_file.fields.fields;

    let mut fields_string = String::new();

    for i in 0..field_count {
        let field = &fields[i as usize];
        fields_string.push('\t');
        fields_string
            .push_str(generate_field_signature(field, &mut class_file.constant_pool).as_str());
        fields_string.push_str(";\n");
    }

    fields_string
}

fn generate_field_signature(
    field: &super::FieldInfo,
    constant_pool: &mut super::ConstantPool,
) -> String {
    let mut field_signature = String::new();

    let access_flags = field.access_flags;

    if access_flags & PUBLIC != 0 {
        field_signature.push_str("public ");
    } else if access_flags & PRIVATE != 0 {
        field_signature.push_str("private ");
    } else if access_flags & PROTECTED != 0 {
        field_signature.push_str("protected ");
    }

    if access_flags & SYNTHETIC != 0 {
        field_signature.push_str("/* synthetic */ ");
    }

    if access_flags & STATIC != 0 {
        field_signature.push_str("static ");
    }

    if access_flags & FINAL != 0 {
        field_signature.push_str("final ");
    }

    if access_flags & VOLATILE != 0 {
        field_signature.push_str("volatile ");
    }

    if access_flags & TRANSIENT != 0 {
        field_signature.push_str("transient ");
    }

    field_signature.push_str(
        format!(
            "{} ",
            super::value::get_type(get_constant_value(field.descriptor_index, constant_pool))
        )
        .as_str(),
    );

    field_signature.push_str(get_constant_value(field.name_index, constant_pool).as_str());

    field_signature
}

fn get_constant_value(index: u16, constant_pool: &mut super::ConstantPool) -> String {
    match constant_pool.get_index(index) {
        super::CPIndexType::Utf8(string) => string,
        _ => panic!("Invalid Type in Constant Pool"),
    }
}
