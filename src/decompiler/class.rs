const PUBLIC: u16 = 0x0001;
const FINAL: u16 = 0x0010;
// TODO: Use this shit lol
// const SUPER: u16 = 0x0020;
const INTERFACE: u16 = 0x0200;
const ABSTRACT: u16 = 0x0400;
const SYNTHETIC: u16 = 0x1000;
const ANNOTATION: u16 = 0x2000;
const ENUM: u16 = 0x4000;

pub fn generate_signature_code(class_file: &mut super::ClassFile) -> String {
    let mut signature = String::new();

    if class_file.access_flags & PUBLIC != 0 {
        signature.push_str("public ");
    }

    if class_file.access_flags & SYNTHETIC != 0 {
        signature.push_str("/* synthetic */");
    }

    if class_file.access_flags & ABSTRACT != 0 {
        if class_file.access_flags & INTERFACE == 0 {
            signature.push_str("abstract ");
        } else {
            if class_file.access_flags & ANNOTATION != 0 {
                signature.push('@');
            }
            signature.push_str("interface ");
        }
    } else if class_file.access_flags & ENUM != 0 {
        signature.push_str("enum ");
    } else {
        if class_file.access_flags & FINAL != 0 {
            signature.push_str("final ");
        }
        signature.push_str("class ")
    }

    signature.push_str(
        format!(
            "{}",
            super::class::get_class_name(class_file.this_class, &mut class_file.constant_pool)
                .split('/')
                .last()
                .unwrap()
        )
        .as_str(),
    );

    signature.push_str(
        format!(
            " extends {} ",
            get_class_name(class_file.super_class, &mut class_file.constant_pool).replace("/", ".")
        )
        .as_str(),
    );

    if class_file.interfaces.interface_count > 0 {
        signature.push_str(
            format!(
                "implements {}",
                super::interface::get_interfaces(class_file)
            )
            .as_str(),
        );
    }

    signature
}

pub fn get_class_name(index: u16, constant_pool: &mut super::ConstantPool) -> String {
    let class_name_index = match constant_pool.get_index(index) {
        super::CPIndexType::Class(a) => a,
        _ => panic!("Invalid Type in Constant Pool"),
    };

    match constant_pool.get_index(class_name_index) {
        super::CPIndexType::Utf8(a) => a,
        _ => panic!("Invalid Type in Constant Pool"),
    }
}
