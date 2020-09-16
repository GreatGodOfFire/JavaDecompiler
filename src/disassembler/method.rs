use super::attribute::AttributeInfo;

#[derive(Debug)]
pub struct MethodPool {
    pub method_count: u16,
    pub methods: Vec<MethodInfo>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}
