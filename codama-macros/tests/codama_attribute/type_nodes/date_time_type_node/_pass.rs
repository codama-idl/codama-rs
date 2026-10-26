use codama_macros::codama;

#[codama(type = date_time(integer(u64)))]
pub struct ImplicitTest;

#[codama(type = date_time(number = integer(u64)))]
pub struct ExplicitTest;

fn main() {}
