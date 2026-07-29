use codama_macros::codama;

#[codama(type = date_time(number(u64)))]
pub struct ImplicitTest;

#[codama(type = date_time(number = number(u64)))]
pub struct ExplicitTest;

#[codama(type = date_time(fixed_size(number(u64), 8)))]
pub struct NestedNumberTest;

fn main() {}
