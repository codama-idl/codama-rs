use codama_macros::codama;

#[codama(enum_discriminator(size = integer(u16)))]
pub struct SizeTest;

#[codama(enum_discriminator(identifier = "banana"))]
pub struct NameTest;

#[codama(enum_discriminator(identifier = "banana", size = integer(u16)))]
pub struct NameAndSizeTest;

fn main() {}
