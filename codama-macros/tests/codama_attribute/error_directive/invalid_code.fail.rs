use codama_macros::codama;

#[codama(error(code = "42"))]
pub struct TestString;

#[codama(error(code = foo(bar)))]
pub struct TestMeta;

fn main() {}
