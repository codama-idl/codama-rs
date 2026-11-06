use codama_macros::codama;

#[codama(type = duration(integer(u64, unit = "ms")))]
pub struct Test;

fn main() {}
