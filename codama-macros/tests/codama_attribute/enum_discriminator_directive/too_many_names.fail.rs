use codama_macros::codama;

#[codama(enum_discriminator(identifier = "apple", identifier = "banana"))]
pub struct Test;

fn main() {}
