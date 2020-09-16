pub mod variable;

mod field;
mod other;
mod store;
mod value;

pub struct MethodInformation {
    pub is_static: bool,
    pub arg_count: u8,
}

// TODO: AThrow, CheckCast, *Cmp*, Dup*, GoTo*, if*, impdep*, Invoke*, Jsr*, Ldc*, LookUpSwitch, Monitor*, Ret, Swap, Tableswitch, Wide

pub fn generate_code(
    instructions: Vec<super::instruction::Instruction>,
    constant_pool: &mut super::ConstantPool,
    method_information: &MethodInformation,
) -> String {
    println!("{:?}", instructions);

    let mut instructions_stack = Vec::<super::instruction::Instruction>::new();
    let mut variable_table: Vec<variable::Variable> = Vec::new();

    let mut code = String::new();

    for instruction in instructions.as_slice() {
        match instruction {
            super::instruction::Instruction::AALoad
            | super::instruction::Instruction::AConstNull
            | super::instruction::Instruction::ALoad0
            | super::instruction::Instruction::ALoad1
            | super::instruction::Instruction::ALoad2
            | super::instruction::Instruction::ALoad3
            | super::instruction::Instruction::ArrayLength
            | super::instruction::Instruction::BALoad
            | super::instruction::Instruction::BiPush(_)
            | super::instruction::Instruction::CALoad
            | super::instruction::Instruction::D2F
            | super::instruction::Instruction::D2I
            | super::instruction::Instruction::D2L
            | super::instruction::Instruction::DAdd
            | super::instruction::Instruction::DALoad
            | super::instruction::Instruction::DCmpG
            | super::instruction::Instruction::DCmpL
            | super::instruction::Instruction::DConst0
            | super::instruction::Instruction::DConst1
            | super::instruction::Instruction::DDiv
            | super::instruction::Instruction::DMul
            | super::instruction::Instruction::DNeg
            | super::instruction::Instruction::DRem
            | super::instruction::Instruction::DSub
            | super::instruction::Instruction::Dup
            | super::instruction::Instruction::DupX1
            | super::instruction::Instruction::DupX2
            | super::instruction::Instruction::Dup2
            | super::instruction::Instruction::Dup2X1
            | super::instruction::Instruction::Dup2X2
            | super::instruction::Instruction::F2D
            | super::instruction::Instruction::F2I
            | super::instruction::Instruction::F2L
            | super::instruction::Instruction::FAdd
            | super::instruction::Instruction::FALoad
            | super::instruction::Instruction::FCmpG
            | super::instruction::Instruction::FCmpL
            | super::instruction::Instruction::FConst0
            | super::instruction::Instruction::FConst1
            | super::instruction::Instruction::FConst2
            | super::instruction::Instruction::FDiv
            | super::instruction::Instruction::FLoad0
            | super::instruction::Instruction::FLoad1
            | super::instruction::Instruction::FLoad2
            | super::instruction::Instruction::FLoad3
            | super::instruction::Instruction::FMul
            | super::instruction::Instruction::FNeg
            | super::instruction::Instruction::FRem
            | super::instruction::Instruction::FSub
            | super::instruction::Instruction::I2C
            | super::instruction::Instruction::I2B
            | super::instruction::Instruction::I2D
            | super::instruction::Instruction::I2F
            | super::instruction::Instruction::I2S
            | super::instruction::Instruction::IAdd
            | super::instruction::Instruction::IALoad
            | super::instruction::Instruction::IAnd
            | super::instruction::Instruction::IConstM1
            | super::instruction::Instruction::IConst0
            | super::instruction::Instruction::IConst1
            | super::instruction::Instruction::IConst2
            | super::instruction::Instruction::IConst3
            | super::instruction::Instruction::IConst4
            | super::instruction::Instruction::IConst5
            | super::instruction::Instruction::IDiv
            | super::instruction::Instruction::ILoad0
            | super::instruction::Instruction::ILoad1
            | super::instruction::Instruction::ILoad2
            | super::instruction::Instruction::ILoad3
            | super::instruction::Instruction::ImpDep1
            | super::instruction::Instruction::ImpDep2
            | super::instruction::Instruction::IMul
            | super::instruction::Instruction::INeg
            | super::instruction::Instruction::IOr
            | super::instruction::Instruction::IRem
            | super::instruction::Instruction::IShl
            | super::instruction::Instruction::IShr
            | super::instruction::Instruction::ISub
            | super::instruction::Instruction::IUShr
            | super::instruction::Instruction::IXor
            | super::instruction::Instruction::L2D
            | super::instruction::Instruction::L2F
            | super::instruction::Instruction::L2I
            | super::instruction::Instruction::LAdd
            | super::instruction::Instruction::LALoad
            | super::instruction::Instruction::LAnd
            | super::instruction::Instruction::LCmp
            | super::instruction::Instruction::LConst0
            | super::instruction::Instruction::LConst1
            | super::instruction::Instruction::Ldc(_)
            | super::instruction::Instruction::LdcW(_)
            | super::instruction::Instruction::Ldc2W(_)
            | super::instruction::Instruction::LDiv
            | super::instruction::Instruction::LLoad0
            | super::instruction::Instruction::LLoad1
            | super::instruction::Instruction::LLoad2
            | super::instruction::Instruction::LLoad3
            | super::instruction::Instruction::LMul
            | super::instruction::Instruction::LNeg
            | super::instruction::Instruction::LOr
            | super::instruction::Instruction::LRem
            | super::instruction::Instruction::LShl
            | super::instruction::Instruction::LShr
            | super::instruction::Instruction::LSub
            | super::instruction::Instruction::LUShr
            | super::instruction::Instruction::LXor
            | super::instruction::Instruction::MonitorEnter
            | super::instruction::Instruction::MonitorExit
            | super::instruction::Instruction::SALoad
            | super::instruction::Instruction::Swap
            | super::instruction::Instruction::ANewArray(_)
            | super::instruction::Instruction::GetStatic(_)
            | super::instruction::Instruction::MultiANewArray(_, _) => {
                instructions_stack.push(*instruction)
            }

            super::instruction::Instruction::PutStatic(_)
            | super::instruction::Instruction::PutField(_) => code.push_str(
                field::decompile_field_instruction(
                    &instruction,
                    &mut instructions_stack,
                    constant_pool,
                    method_information,
                )
                .as_str(),
            ),
            super::instruction::Instruction::AAStore
            | super::instruction::Instruction::AStore(_)
            | super::instruction::Instruction::AStore0
            | super::instruction::Instruction::AStore1
            | super::instruction::Instruction::AStore2
            | super::instruction::Instruction::AStore3
            | super::instruction::Instruction::BAStore
            | super::instruction::Instruction::CAStore
            | super::instruction::Instruction::DAStore
            | super::instruction::Instruction::DStore(_)
            | super::instruction::Instruction::DStore0
            | super::instruction::Instruction::DStore1
            | super::instruction::Instruction::DStore2
            | super::instruction::Instruction::DStore3
            | super::instruction::Instruction::FAStore
            | super::instruction::Instruction::FStore(_)
            | super::instruction::Instruction::FStore0
            | super::instruction::Instruction::FStore1
            | super::instruction::Instruction::FStore2
            | super::instruction::Instruction::FStore3
            | super::instruction::Instruction::IAStore
            | super::instruction::Instruction::IStore(_)
            | super::instruction::Instruction::IStore0
            | super::instruction::Instruction::IStore1
            | super::instruction::Instruction::IStore2
            | super::instruction::Instruction::IStore3
            | super::instruction::Instruction::LAStore
            | super::instruction::Instruction::LStore(_)
            | super::instruction::Instruction::LStore0
            | super::instruction::Instruction::LStore1
            | super::instruction::Instruction::LStore2
            | super::instruction::Instruction::LStore3
            | super::instruction::Instruction::SAStore => code.push_str(
                store::decompile_store_instruction(
                    instruction,
                    &mut instructions_stack,
                    constant_pool,
                    method_information,
                    &mut variable_table,
                )
                .as_str(),
            ),

            super::instruction::Instruction::Return => code.push_str("return;"),
            super::instruction::Instruction::AReturn
            | super::instruction::Instruction::DReturn
            | super::instruction::Instruction::FReturn
            | super::instruction::Instruction::IReturn
            | super::instruction::Instruction::LReturn
            | super::instruction::Instruction::AThrow
            | super::instruction::Instruction::Breakpoint
            | super::instruction::Instruction::Nop
            | super::instruction::Instruction::Pop
            | super::instruction::Instruction::Pop2 => code.push_str(
                other::decompile_instruction(
                    instruction,
                    &mut instructions_stack,
                    constant_pool,
                    method_information,
                )
                .as_str(),
            ),

            _ => {}
        }
    }
    if !instructions_stack.is_empty() {
        code.push_str("\n// Instructions Stack is not empty.\n");
        code.push_str(
            format!("// Instructions Stack Dump: {:#?}\n", instructions_stack)
                .replace('\n', "\n// ")
                .as_str(),
        );
        code.push_str(
            format!("Instructions Dump: {:#?}", &instructions.as_slice())
                .replace('\n', "\n// ")
                .as_str(),
        )
    }
    code
}
