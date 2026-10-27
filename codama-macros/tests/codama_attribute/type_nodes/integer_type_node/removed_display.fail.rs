use codama_macros::codama;

#[codama(type = integer(i64, display = date_time))]
pub struct DateTimeTest;

#[codama(type = integer(u64, display = duration))]
pub struct DurationTest;

fn main() {}
