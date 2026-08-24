use codama_macros::codama;

#[codama(remaining_accounts(argument("signers"), banana))]
pub struct Test;

fn main() {}
