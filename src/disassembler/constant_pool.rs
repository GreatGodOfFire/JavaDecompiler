#[derive(Debug)]
pub struct ConstantPool {
    pub index_count: u16,
    pub indexes: Vec<CPIndexType>,
}

impl ConstantPool {
    pub fn new(index_count: u16, indexes: Vec<CPIndexType>) -> Self {
        ConstantPool {
            index_count,
            indexes,
        }
    }

    pub fn get_index(&self, index: u16) -> CPIndexType {
        self.indexes[(index - 1) as usize].clone()
    }
}

#[derive(Debug)]
pub enum CPIndexType {
    Class(u16),
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    String(u16),
    Integer(u32),
    Float(f32),
    Long(u64),
    Double(f64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8(String),
    MethodHandle {
        reference_type: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        boostrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl Clone for CPIndexType {
    fn clone(&self) -> Self {
        match self {
            CPIndexType::Class(a) => CPIndexType::Class(a.clone()),
            CPIndexType::FieldRef {
                class_index,
                name_and_type_index,
            } => CPIndexType::FieldRef {
                class_index: class_index.clone(),
                name_and_type_index: name_and_type_index.clone(),
            },
            CPIndexType::MethodRef {
                class_index,
                name_and_type_index,
            } => CPIndexType::MethodRef {
                class_index: class_index.clone(),
                name_and_type_index: name_and_type_index.clone(),
            },
            CPIndexType::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            } => CPIndexType::InterfaceMethodRef {
                class_index: class_index.clone(),
                name_and_type_index: name_and_type_index.clone(),
            },
            CPIndexType::String(value) => CPIndexType::String(value.clone()),
            CPIndexType::Integer(value) => CPIndexType::Integer(value.clone()),
            CPIndexType::Float(value) => CPIndexType::Float(value.clone()),
            CPIndexType::Long(value) => CPIndexType::Long(*value),
            CPIndexType::Double(value) => CPIndexType::Double(*value),
            CPIndexType::NameAndType {
                name_index,
                descriptor_index,
            } => CPIndexType::NameAndType {
                name_index: name_index.clone(),
                descriptor_index: descriptor_index.clone(),
            },
            CPIndexType::Utf8(a) => CPIndexType::Utf8(a.to_string()),
            CPIndexType::MethodHandle {
                reference_type,
                reference_index,
            } => CPIndexType::MethodHandle {
                reference_type: reference_type.clone(),
                reference_index: reference_index.clone(),
            },
            CPIndexType::MethodType {
                descriptor_index,
            } => CPIndexType::MethodType {
                descriptor_index: descriptor_index.clone(),
            },
            CPIndexType::InvokeDynamic {
                boostrap_method_attr_index,
                name_and_type_index,
            } => CPIndexType::InvokeDynamic {
                boostrap_method_attr_index: boostrap_method_attr_index.clone(),
                name_and_type_index: name_and_type_index.clone(),
            },
        }
    }
}