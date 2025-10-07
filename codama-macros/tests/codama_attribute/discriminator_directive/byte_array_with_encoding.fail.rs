use codama_macros::codama;

#[codama(discriminator(bytes = [1, 2, 3, 4], encoding = "utf8"))]
pub struct TestByteArrayAndEncoding;

#[codama(discriminator(encoding = "utf8", bytes = [1, 2, 3, 4]))]
pub struct TestEncodingAndByteArray;

fn main() {}
