use super::super::*;

pub fn decompile_store_instruction(
    instruction: &instruction::Instruction,
    instructions_stack: &mut Vec<instruction::Instruction>,
    constant_pool: &mut ConstantPool,
    method_information: &super::MethodInformation,
    variable_table: &mut Vec<super::variable::Variable>,
) -> String {
    match instruction {
        instruction::Instruction::AAStore => {
            let value =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let index =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let arrayref =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            format!("{}[{}] = {};\n", arrayref.0, index.0, value.0)
        }

        instruction::Instruction::AStore(var_num) => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(
                get_var(
                    method_information,
                    variable_table,
                    &objectref,
                    *var_num as u16,
                )
                .as_str(),
            );

            code.push_str(format!(" = {}", objectref.0).as_str());

            code
        }

        instruction::Instruction::AStore0 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 0).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::AStore1 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 1).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::AStore2 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 2).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::AStore3 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 3).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::BAStore => {
            let value =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let index =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let arrayref =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            format!("{}[{}] = {};\n", arrayref.0, index.0, value.0)
        }

        instruction::Instruction::CAStore => {
            let value =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let index =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let arrayref =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            format!("{}[{}] = {};\n", arrayref.0, index.0, value.0)
        }

        instruction::Instruction::DAStore => {
            let value =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let index =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let arrayref =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            format!("{}[{}] = {};\n", arrayref.0, index.0, value.0)
        }

        instruction::Instruction::DStore(var_num) => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(
                get_var(
                    method_information,
                    variable_table,
                    &objectref,
                    *var_num as u16,
                )
                .as_str(),
            );

            code.push_str(format!(" = {}", objectref.0).as_str());

            code
        }

        instruction::Instruction::DStore0 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 0).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::DStore1 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 1).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::DStore2 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 2).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }
        
        instruction::Instruction::DStore3 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 3).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::FAStore => {
            let value =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let index =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let arrayref =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            format!("{}[{}] = {};\n", arrayref.0, index.0, value.0)
        }

        instruction::Instruction::FStore(var_num) => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(
                get_var(
                    method_information,
                    variable_table,
                    &objectref,
                    *var_num as u16,
                )
                .as_str(),
            );

            code.push_str(format!(" = {}", objectref.0).as_str());

            code
        }

        instruction::Instruction::FStore0 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 0).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::FStore1 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 1).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::FStore2 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 2).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }
        
        instruction::Instruction::FStore3 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 3).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::IAStore => {
            let value =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let index =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let arrayref =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            format!("{}[{}] = {};\n", arrayref.0, index.0, value.0)
        }

        instruction::Instruction::IStore(var_num) => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(
                get_var(
                    method_information,
                    variable_table,
                    &objectref,
                    *var_num as u16,
                )
                .as_str(),
            );

            code.push_str(format!(" = {}", objectref.0).as_str());

            code
        }

        instruction::Instruction::IStore0 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 0).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::IStore1 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 1).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::IStore2 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 2).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }
        
        instruction::Instruction::IStore3 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 3).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::LAStore => {
            let value =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let index =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let arrayref =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            format!("{}[{}] = {};\n", arrayref.0, index.0, value.0)
        }

        instruction::Instruction::LStore(var_num) => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(
                get_var(
                    method_information,
                    variable_table,
                    &objectref,
                    *var_num as u16,
                )
                .as_str(),
            );

            code.push_str(format!(" = {}", objectref.0).as_str());

            code
        }

        instruction::Instruction::LStore0 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 0).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::LStore1 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 1).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::LStore2 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 2).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }
        
        instruction::Instruction::LStore3 => {
            let objectref =
                super::value::get_value(instructions_stack, constant_pool, method_information);

            let mut code = String::new();

            code.push_str(get_var(method_information, variable_table, &objectref, 3).as_str());

            code.push_str(format!(" = {};\n", objectref.0).as_str());

            code
        }

        instruction::Instruction::SAStore => {
            let value =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let index =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            let arrayref =
                super::value::get_value(instructions_stack, constant_pool, method_information);
            format!("{}[{}] = {};\n", arrayref.0, index.0, value.0)
        }

        instruction => {
            panic!("Invalid Instruction given: {:?}", instruction)
            // String::new()
        }
    }
}

fn get_var(
    method_information: &super::MethodInformation,
    variable_table: &mut Vec<super::variable::Variable>,
    objectref: &(String, super::value::Type),
    var_num: u16,
) -> String {
    let mut exists = false;
    for var in variable_table.as_slice() {
        if var.var_num == 0 {
            exists = true;
        }
    }

    let mut code = String::new();

    if exists {
        if method_information.arg_count as u16 > var_num {
            if method_information.is_static {
                code.push_str(format!("arg{}", var_num).as_str());
            } else {
                if var_num == 0 {
                    code.push_str("this");
                } else {
                    code.push_str(format!("arg{}", var_num).as_str());
                }
            }
        } else {
            code.push_str(format!("var{}", var_num).as_str());
        }
    // if method_information.is_static {
    //     if method_information.arg_count as u16 >= var_num {
    //         code.push_str(format!("arg{}", var_num).as_str());
    //     } else {
    //         code.push_str("var0");
    //     }
    // } else {
    //     code.push_str("this");
    // }
    } else {
        code.push_str(format!("{} var{}", objectref.1.clone(), var_num).as_str());
        variable_table.push(super::variable::Variable {
            ty: objectref.1.clone(),
            var_num,
        });
    }

    code
}
