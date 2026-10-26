use codama_macros::codama;

#[codama(remaining_accounts(argument("signers"), signer))]
pub struct Test;

fn main() {}
