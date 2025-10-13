use codama_macros::codama;

#[codama(type = field("age", number(u8), "years"))]
pub struct Test;

#[codama(type = field(name = "age", type = number(u8), name = "years"))]
pub struct TestExplicit;

fn main() {}
