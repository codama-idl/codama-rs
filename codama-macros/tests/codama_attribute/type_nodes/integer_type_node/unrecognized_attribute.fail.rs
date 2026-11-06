use codama_macros::codama;

#[codama(type = integer(u32, unrecognized = 42, le))]
pub struct Test;

fn main() {}
