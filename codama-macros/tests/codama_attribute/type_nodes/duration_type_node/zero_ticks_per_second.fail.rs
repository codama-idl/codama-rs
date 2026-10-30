use codama_macros::codama;

#[codama(type = duration(integer(u64), ticks_per_second = 0))]
pub struct Test;

fn main() {}
