use codama_macros::codama;

#[codama(provide("decimals", account_field("mint", "decimals")))]
pub struct Test;

fn main() {}
