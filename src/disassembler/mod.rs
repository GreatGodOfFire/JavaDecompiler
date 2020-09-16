pub mod attribute;
pub mod builder;
pub mod constant_pool;
pub mod field;
pub mod interface;
pub mod method;

#[derive(Debug)]
pub struct ClassFile {
    magic: u32,
    minor: u16,
    major: u16,
    pub constant_pool: constant_pool::ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: interface::InterfacePool,
    pub fields: field::FieldPool,
    pub methods: method::MethodPool,
    pub attributes: attribute::AttributePool,
}

impl ClassFile {
    pub fn new(buf: &[u8]) -> ClassFile {
        builder::ClassFileBuilder {
            buf,
            constant_pool: constant_pool::ConstantPool::new(
                0,
                Vec::<constant_pool::CPIndexType>::new(),
            ),
        }
        .parse()
    }
}
