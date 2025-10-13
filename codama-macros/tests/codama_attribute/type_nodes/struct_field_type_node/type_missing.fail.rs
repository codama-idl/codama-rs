use codama_macros::codama;

#[codama(type = field("age"))]
pub struct Test;

#[codama(type = field(name = "age"))]
pub struct TestExplicit;

fn main() {}
