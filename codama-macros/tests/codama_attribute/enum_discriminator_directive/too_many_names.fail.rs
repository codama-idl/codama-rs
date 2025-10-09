use codama_macros::codama;

#[codama(enum_discriminator(name = "apple", name = "banana"))]
pub struct Test;

fn main() {}
