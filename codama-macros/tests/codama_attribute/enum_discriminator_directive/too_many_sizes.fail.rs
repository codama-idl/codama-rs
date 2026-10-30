use codama_macros::codama;

#[codama(enum_discriminator(size = integer(u16), size = integer(u32)))]
pub struct Test;

fn main() {}
