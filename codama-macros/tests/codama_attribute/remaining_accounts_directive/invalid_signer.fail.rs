use codama_macros::codama;

#[codama(remaining_accounts("signers", signer = invalid))]
pub struct Test;

fn main() {}
