use codama_macros::codama;

#[codama(remaining_accounts(argument("signers"), signer = invalid))]
pub struct Test;

fn main() {}
