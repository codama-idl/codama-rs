use super::TypeNodeTrait;
use crate::NodeTrait;

pub use NumberFormat::*;

#[derive(Debug)]
pub struct NumberTypeNode {
    // Data.
    pub format: NumberFormat,
    pub endian: Endian,
}

impl NumberTypeNode {
    pub fn new(format: NumberFormat, endian: Endian) -> Self {
        Self { format, endian }
    }
}

impl TypeNodeTrait for NumberTypeNode {}
impl NodeTrait for NumberTypeNode {
    const KIND: &'static str = "numberTypeNode";
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

#[derive(Debug)]
pub enum Endian {
    Big,
    Little,
}
