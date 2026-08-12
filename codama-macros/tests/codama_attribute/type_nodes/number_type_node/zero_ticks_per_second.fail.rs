use codama_macros::codama;

#[codama(type = number(i64, display = date_time(ticks_per_second = 0)))]
pub struct Test;

fn main() {}
