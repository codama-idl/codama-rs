use codama_macros::codama;

#[codama(type = date_time(integer(i64), ticks_per_second = 0))]
pub struct Test;

fn main() {}
