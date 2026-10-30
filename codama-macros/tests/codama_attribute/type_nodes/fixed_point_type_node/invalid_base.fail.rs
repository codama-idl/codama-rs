use codama_macros::codama;

#[codama(type = fixed_point(integer(u64), scale = 6, base = 16))]
pub struct Test;

fn main() {}
