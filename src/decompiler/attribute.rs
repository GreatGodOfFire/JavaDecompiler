#[derive(Debug)]
pub struct Attribute {
    name_index: u16,
    length: u32,
    bytes: Vec<u8>,
}

impl Attribute {
    pub fn new(name_index: u16, length: u32, bytes: Vec<u8>) -> Self {
        Attribute {
            name_index,
            length,
            bytes,
        }
    }

    pub fn get_name_index(&self) -> u16 {
        self.name_index
    }
}
