use crate::Node;

pub use NumberFormat::*;

pub trait NumberTypeNodeFlag {
    fn get_number_type_node(&self) -> &NumberTypeNode;
}

impl NumberTypeNodeFlag for NumberTypeNode {
    fn get_number_type_node(&self) -> &NumberTypeNode {
        self
    }
}

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

impl Node for NumberTypeNode {
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
