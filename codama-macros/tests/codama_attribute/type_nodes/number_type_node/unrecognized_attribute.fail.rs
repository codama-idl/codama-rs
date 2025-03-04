use codama_macros::codama;

#[codama(type = number(u32, unrecognized = 42, le))]
pub struct Test;

fn main() {}
