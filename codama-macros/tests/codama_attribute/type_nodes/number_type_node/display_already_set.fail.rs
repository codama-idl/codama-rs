use codama_macros::codama;

#[codama(type = number(u64, display = amount, display = date_time))]
pub struct Test;

fn main() {}
