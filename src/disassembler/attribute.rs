#[derive(Debug)]
pub struct AttributePool {
    pub attribute_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: AttributeNameIndex,
    pub attribute_length: u32,
    pub info: Vec<u8>,
}

#[derive(Debug)]
pub struct AttributeNameIndex {
    pub value: super::constant_pool::CPIndexType,
    pub index: u16,
}
