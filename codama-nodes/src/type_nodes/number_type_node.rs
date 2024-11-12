#[derive(Debug)]
pub struct NumberTypeNode {
    pub format: NumberFormat,
    pub endian: Endian,
}

impl NumberTypeNode {
    pub fn new(format: NumberFormat, endian: Endian) -> Self {
        Self { format, endian }
    }
}

#[derive(Debug)]
pub enum NumberFormat {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    ShortU16,
}

pub use NumberFormat::*;

#[derive(Debug)]
pub enum Endian {
    Big,
    Little,
}
