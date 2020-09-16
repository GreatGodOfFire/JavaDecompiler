mod decompiler;
mod disassembler;

use std::fs;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let file = fs::File::open("Test.class")?;
    // let file = fs::File::open("샒㕱쪪옸䤍칗.class")?;
    let mut reader = io::BufReader::new(file);
    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf)?;
    let mut class_file = disassembler::ClassFile::new(buf.as_slice());

    println!("{}", decompiler::decompile_class_file(&mut class_file));
    Ok(())
}
