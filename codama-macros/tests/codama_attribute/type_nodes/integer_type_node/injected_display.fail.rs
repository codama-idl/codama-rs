use codama_macros::codama;

#[codama(type = integer(u64, display = injected("amount")))]
pub struct Test;

fn main() {}
