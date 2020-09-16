#[derive(Debug)]
pub struct ExceptionTableIndex {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub fn read_exception_table_index(index: &[u8; 8]) -> ExceptionTableIndex {
    let start_pc = u16::from_be_bytes([index[0], index[1]]);
    let end_pc = u16::from_be_bytes([index[2], index[3]]);
    let handler_pc = u16::from_be_bytes([index[4], index[5]]);
    let catch_type = u16::from_be_bytes([index[6], index[7]]);

    ExceptionTableIndex {
        start_pc,
        end_pc,
        handler_pc,
        catch_type
    }
}