use codama_macros::codama;

#[codama(discriminator(size = 100))]
pub struct SizeTest;

#[codama(discriminator(field = "discriminator"))]
#[codama(discriminator(field = "discriminator", offset = 42))]
pub struct FieldTest;

#[codama(discriminator(bytes = [1, 2, 3, 4]))]
#[codama(discriminator(bytes = [1, 2, 3, 4], offset = 42))]
#[codama(discriminator(bytes = "01020304"))]
#[codama(discriminator(bytes = "hello", encoding = "utf8"))]
#[codama(discriminator(bytes = "HeLLo", encoding = "base58", offset = 42))]
pub struct BytesTest;

fn main() {}
