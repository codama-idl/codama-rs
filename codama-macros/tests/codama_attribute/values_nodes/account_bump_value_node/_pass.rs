use codama_macros::codama;

#[codama(default_value = account_bump(name = "escrow"))]
pub struct TestExplicit;

#[codama(default_value = account_bump("escrow"))]
pub struct TestImplicit;

fn main() {}
