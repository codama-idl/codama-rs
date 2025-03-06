use codama_macros::codama;

#[codama(error("hello", "world"))]
pub struct Test;

#[codama(error(message = "hello", message = "world"))]
pub struct TestExplicit;

fn main() {}
