use super::super::*;

pub fn decompile_field_instruction(
    instruction: &instruction::Instruction,
    instructions_stack: &mut Vec<instruction::Instruction>,
    constant_pool: &mut ConstantPool,
    method_information: &super::MethodInformation
) -> String {
    let mut code = String::new();
    match instruction {
        instruction::Instruction::PutStatic(index) => {
            let field = match constant_pool.get_index(*index) {
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

            let value = super::value::get_value(instructions_stack, constant_pool, method_information);

            code.push_str(format!("{}.{} = {};", class_name, name, value.0).as_str());
            code.push('\n');
        }
        instruction::Instruction::PutField(index) => {
            let field = match constant_pool.get_index(*index) {
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

            let value = super::value::get_value(instructions_stack, constant_pool, method_information);

            code.push_str(format!("{}.{} = {};", class_name, name, value.0).as_str());
            code.push('\n');
        }
        _ => panic!("Invalid Instruction given"),
    };
    code
}
