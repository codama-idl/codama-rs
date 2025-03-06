use codama_macros::codama;

#[codama(error(message = 42))]
pub struct TestInteger;

#[codama(error(message = foo(bar)))]
pub struct TestMeta;

fn main() {}
