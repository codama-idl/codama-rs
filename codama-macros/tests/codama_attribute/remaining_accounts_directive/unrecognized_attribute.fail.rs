use codama_macros::codama;

#[codama(remaining_accounts("signers", banana))]
pub struct Test;

fn main() {}
