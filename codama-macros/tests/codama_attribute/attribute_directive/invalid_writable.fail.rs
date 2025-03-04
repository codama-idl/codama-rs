use codama_macros::codama;

#[codama(account(writable = invalid))]
pub struct Test;

fn main() {}
