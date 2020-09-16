pub fn get_interfaces(class_file: &mut super::ClassFile) -> String {
    let mut interfaces_string = String::new();

    for i in 0..class_file.interfaces.interface_count {
        interfaces_string.push_str(
            format!(
                "{} ",
                get_interface_name(
                    class_file.interfaces.interfaces[i as usize],
                    &mut class_file.constant_pool
                )
                .replace("/", ".")
            )
            .as_str(),
        );
    }

    interfaces_string
}

fn get_interface_name(index: u16, constant_pool: &mut super::ConstantPool) -> String {
    let class_name_index = match constant_pool.get_index(index) {
        super::CPIndexType::Class(a) => a,
        _ => panic!("Invalid Type in Constant Pool"),
    };

    match constant_pool.get_index(class_name_index) {
        super::CPIndexType::Utf8(a) => a,
        _ => panic!("Invalid Type in Constant Pool"),
    }
}
