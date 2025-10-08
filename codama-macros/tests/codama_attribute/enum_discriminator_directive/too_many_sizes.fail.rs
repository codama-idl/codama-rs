use codama_macros::codama;

#[codama(enum_discriminator(size = number(u16), size = number(u32)))]
pub struct Test;

fn main() {}
