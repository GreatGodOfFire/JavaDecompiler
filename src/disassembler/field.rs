use super::attribute::AttributeInfo;

#[derive(Debug)]
pub struct FieldPool {
    pub field_count: u16,
    pub fields: Vec<FieldInfo>,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}
