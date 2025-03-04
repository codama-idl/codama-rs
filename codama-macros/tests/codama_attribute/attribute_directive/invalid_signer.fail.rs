use codama_macros::codama;

#[codama(account(signer = invalid))]
pub struct Test;

fn main() {}
