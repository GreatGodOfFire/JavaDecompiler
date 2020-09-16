pub fn get_type(value_string: String) -> String {
    match value_string
        .get(0..1)
        .expect("Invalid or empty String given")
    {
        "B" => "byte".to_string(),
        "C" => "char".to_string(),
        "D" => "double".to_string(),
        "F" => "float".to_string(),
        "I" => "int".to_string(),
        "J" => "long".to_string(),
        "L" => value_string
            .get(1..value_string.len())
            .expect("Invalid String given")
            .replace("/", ".")
            .replace(";", ""),

        "S" => "short".to_string(),
        "V" => "void".to_string(),
        "Z" => "boolean".to_string(),
        "[" => {
            let mut field_type = get_type(
                value_string
                    .get(1..value_string.len())
                    .expect("Invalid String given")
                    .to_string(),
            );
            field_type.push_str("[]");
            field_type
        }
        _ => unimplemented!("Invalid type given"),
    }
}
