use codama_macros::codama;

#[codama(default_value = account(name = "authority"))]
pub struct TestExplicit;

#[codama(default_value = account("authority"))]
pub struct TestImplicit;

fn main() {}
