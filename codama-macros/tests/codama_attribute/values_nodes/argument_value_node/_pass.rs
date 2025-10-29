use codama_macros::codama;

#[codama(default_value = argument(name = "amount"))]
pub struct TestExplicit;

#[codama(default_value = argument("amount"))]
pub struct TestImplicit;

fn main() {}
