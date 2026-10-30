use codama_macros::codama;

#[codama(type = date_time(integer(i64, display = amount(decimals = 3))))]
pub struct Test;

fn main() {}
