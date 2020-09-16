mod attribute;
mod class;
mod codegen;
mod field;
mod instruction;
mod interface;
mod method;
mod value;
mod variable;

pub use super::disassembler::constant_pool::*;
pub use super::disassembler::field::FieldInfo;
pub use super::disassembler::method::*;
pub use super::disassembler::ClassFile;

pub fn decompile_class_file(class_file: &mut ClassFile) -> String {
    let mut code = String::new();

    code.push_str(class::generate_signature_code(class_file).as_str());

    code.push_str("{\n");

    code.push_str(field::decompile_fields(class_file).as_str());

    code.push('\n');

    code.push_str(method::decompile_methods(class_file).as_str());

    code.push_str("}\n");

    code
}
