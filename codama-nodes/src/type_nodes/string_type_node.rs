use super::TypeNodeTrait;
use crate::NodeTrait;

pub use BytesEncoding::*;

#[derive(Debug)]
pub struct StringTypeNode {
    // Data.
    pub encoding: BytesEncoding,
}

impl StringTypeNode {
    pub fn new(encoding: BytesEncoding) -> Self {
        Self { encoding }
    }
}

impl TypeNodeTrait for StringTypeNode {}
impl NodeTrait for StringTypeNode {
    const KIND: &'static str = "stringTypeNode";
}

#[derive(Debug)]
pub enum BytesEncoding {
    Base16,
    Base58,
    Base64,
    Utf8,
}
