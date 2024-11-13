use codama_nodes_derive::{Node, TypeNode};

pub use NumberFormat::*;

#[derive(Node, TypeNode, Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endian {
    Big,
    Little,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = NumberTypeNode::new(U8, Endian::Big);
        assert_eq!(node.format, NumberFormat::U8);
        assert_eq!(node.endian, Endian::Big);
    }
}
