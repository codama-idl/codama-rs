use codama_macros::codama;

#[codama(type = number(u64, display = injected("amount")))]
pub struct Test;

fn main() {}
