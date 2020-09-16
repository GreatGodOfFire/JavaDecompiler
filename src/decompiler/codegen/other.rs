use super::super::*;

pub fn decompile_instruction(
    instruction: &instruction::Instruction,
    instructions_stack: &mut Vec<instruction::Instruction>,
    constant_pool: &mut ConstantPool,
    method_information: &super::MethodInformation,
    // variable_table: &mut Vec<super::variable::Variable>,
) -> String {

    let mut code = String::new();

    match instruction {
        instruction::Instruction::Return => code.push_str("return;"),
        instruction::Instruction::AReturn 
        | instruction::Instruction::DReturn
        | instruction::Instruction::FReturn
        | instruction::Instruction::IReturn
        | instruction::Instruction::LReturn => {
            let mut return_string = String::from("return ");
            return_string.push_str(
                super::value::get_value(instructions_stack, constant_pool, method_information)
                    .0
                    .as_str(),
            );
            code.push_str(return_string.as_str());
            code.push(';');
        }
        instruction::Instruction::AThrow => {
            let exceptionref = super::value::get_value(instructions_stack, constant_pool, method_information);
            code.push_str(format!("throw {};", exceptionref.0).as_str());
        }
        instruction::Instruction::Breakpoint => {
            code.push_str("// Breakpoint");
        }

        instruction::Instruction::Nop => {
            code.push_str("// Nop");
        }

        instruction::Instruction::Pop => {
            let _ = super::value::get_value(instructions_stack, constant_pool, method_information);
        }

        instruction::Instruction::Pop2 => {
            let val = super::value::get_value(instructions_stack, constant_pool, method_information);

            if val.1 != super::value::Type::Long || val.1 != super::value::Type::Double {
                let _ = super::value::get_value(instructions_stack, constant_pool, method_information);
            }
        }
        _ => unreachable!()
    }

    String::new()
}
