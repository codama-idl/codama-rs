#[derive(Debug)]
pub struct StringTypeNode {
    pub encoding: BytesEncoding,
}

#[derive(Debug)]
pub enum BytesEncoding {
    Base16,
    Base58,
    Base64,
    Utf8,
}
