use codama::codama;

#[codama(account)]
pub struct Test;

#[codama(account(signer, writable))]
pub struct TestWithBooleans;

fn main() {}
