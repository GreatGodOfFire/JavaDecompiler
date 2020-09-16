use super::attribute::*;
use super::constant_pool::*;
use super::field::*;
use super::interface::InterfacePool;
use super::method::*;

use super::ClassFile;

use std::io::Read;
pub struct ClassFileBuilder<'a> {
    pub buf: &'a [u8],
    pub constant_pool: ConstantPool,
}

impl<'a> ClassFileBuilder<'a> {
    pub fn parse(mut self) -> ClassFile {
        let header = (self.read_u32(), self.read_u16(), self.read_u16());
        self.constant_pool = self.read_constant_pool();
        ClassFile {
            magic: header.0,
            minor: header.1,
            major: header.2,
            access_flags: self.read_u16(),
            this_class: self.read_u16(),
            super_class: self.read_u16(),
            interfaces: self.read_interface_pool(),
            fields: self.read_field_pool(),
            methods: self.read_method_pool(),
            attributes: self.read_attribute_pool(),
            constant_pool: self.constant_pool,
        }
    }
}

// value read methods
impl<'a> ClassFileBuilder<'a> {
    pub fn read_constant_pool(&mut self) -> ConstantPool {
        let index_count = self.read_u16();
        let mut indexes = Vec::new();
        let mut push_twice = false;
        let mut i = 1;
        while i < index_count {
            indexes.push(match self.read_u8() {
                1 => {
                    let mut buffer = vec![0_u8; self.read_u16() as usize];
                    self.buf.read_exact(&mut buffer).unwrap();
                    CPIndexType::Utf8(String::from_utf8(buffer).expect("Invalid UTF8 String"))
                }
                3 => CPIndexType::Integer(self.read_u32()),
                4 => CPIndexType::Float(self.read_u32() as f32),
                5 => {
                    push_twice = true;
                    CPIndexType::Long({
                        let mut buffer = [0; 8];
                        self.buf.read_exact(&mut buffer).unwrap();
                        u64::from_be_bytes(buffer)
                    })
                }
                6 => {
                    push_twice = true;
                    CPIndexType::Double(self.read_f64())
                }
                7 => CPIndexType::Class(self.read_u16()),
                8 => CPIndexType::String(self.read_u16()),
                9 => CPIndexType::FieldRef {
                    class_index: self.read_u16(),
                    name_and_type_index: self.read_u16(),
                },
                10 => CPIndexType::MethodRef {
                    class_index: self.read_u16(),
                    name_and_type_index: self.read_u16(),
                },
                11 => CPIndexType::InterfaceMethodRef {
                    class_index: self.read_u16(),
                    name_and_type_index: self.read_u16(),
                },
                12 => CPIndexType::NameAndType {
                    name_index: self.read_u16(),
                    descriptor_index: self.read_u16(),
                },
                15 => CPIndexType::MethodHandle {
                    reference_type: self.read_u8(),
                    reference_index: self.read_u16(),
                },
                16 => CPIndexType::MethodType {
                    descriptor_index: self.read_u16(),
                },
                18 => CPIndexType::InvokeDynamic {
                    boostrap_method_attr_index: self.read_u16(),
                    name_and_type_index: self.read_u16(),
                },
                _ => unimplemented!("Unknown tag"),
            });
            if push_twice {
                indexes.push(CPIndexType::Long(0));
                i += 1;
                push_twice = false;
            }
            i += 1;
        }

        ConstantPool {
            index_count,
            indexes,
        }
    }

    pub fn read_interface_pool(&mut self) -> InterfacePool {
        let interface_count = self.read_u16();
        let mut interfaces = Vec::new();

        for _i in 0..interface_count {
            interfaces.push(self.read_u16())
        }

        InterfacePool {
            interface_count,
            interfaces,
        }
    }

    pub fn read_field_pool(&mut self) -> FieldPool {
        let field_count = self.read_u16();
        let mut fields: Vec<FieldInfo> = Vec::new();

        for _i in 0..field_count {
            let access_flags = self.read_u16();
            let name_index = self.read_u16();
            let descriptor_index = self.read_u16();
            let attributes_count = self.read_u16();
            let mut attributes: Vec<AttributeInfo> = Vec::new();

            for _j in 0..attributes_count {
                attributes.push(self.read_attribute());
            }

            fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        FieldPool {
            field_count,
            fields,
        }
    }

    pub fn read_method_pool(&mut self) -> MethodPool {
        let method_count = self.read_u16();
        let mut methods: Vec<MethodInfo> = Vec::new();

        for _i in 0..method_count {
            let access_flags = self.read_u16();
            let name_index = self.read_u16();
            let descriptor_index = self.read_u16();
            let attributes_count = self.read_u16();
            let mut attributes: Vec<AttributeInfo> = Vec::new();

            for _j in 0..attributes_count {
                attributes.push(self.read_attribute())
            }

            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        MethodPool {
            method_count,
            methods,
        }
    }

    pub fn read_attribute_pool(&mut self) -> AttributePool {
        let attribute_count = self.read_u16();
        let mut attributes = Vec::new();

        for _i in 0..attribute_count {
            attributes.push(self.read_attribute());
        }
        AttributePool {
            attribute_count,
            attributes,
        }
    }

    fn read_attribute(&mut self) -> AttributeInfo {
        let attribute_name_index = self.read_u16();
        let attribute_length = self.read_u32();
        let mut info: Vec<u8> = Vec::new();

        for _j in 0..attribute_length {
            info.push(self.read_u8());
        }

        let value = self.constant_pool.get_index(attribute_name_index);

        AttributeInfo {
            attribute_name_index: AttributeNameIndex {
                value: value,
                index: attribute_name_index,
            },
            attribute_length,
            info,
        }
    }
}

// byte read methods
impl<'a> ClassFileBuilder<'a> {
    fn read_u8(&mut self) -> u8 {
        let mut buffer = [0u8];
        self.buf.read_exact(&mut buffer).unwrap();
        u8::from_be_bytes(buffer)
    }

    fn read_u16(&mut self) -> u16 {
        let mut buffer = [0; 2];
        self.buf.read_exact(&mut buffer).unwrap();
        u16::from_be_bytes(buffer)
    }

    fn read_u32(&mut self) -> u32 {
        let mut buffer = [0; 4];
        self.buf.read_exact(&mut buffer).unwrap();
        u32::from_be_bytes(buffer)
    }

    fn read_f64(&mut self) -> f64 {
        let mut buffer = [0; 8];
        self.buf.read_exact(&mut buffer).unwrap();
        f64::from_be_bytes(buffer)
    }
}
