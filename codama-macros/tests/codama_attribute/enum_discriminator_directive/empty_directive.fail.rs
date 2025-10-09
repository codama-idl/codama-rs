use codama_macros::codama;

#[codama(enum_discriminator)]
pub struct EmptyTest;

#[codama(enum_discriminator())]
pub struct EmptyTestWithBraces;

fn main() {}
