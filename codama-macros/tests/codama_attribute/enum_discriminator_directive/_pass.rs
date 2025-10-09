use codama_macros::codama;

#[codama(enum_discriminator(size = number(u16)))]
pub struct SizeTest;

#[codama(enum_discriminator(name = "banana"))]
pub struct NameTest;

#[codama(enum_discriminator(name = "banana", size = number(u16)))]
pub struct NameAndSizeTest;

fn main() {}
