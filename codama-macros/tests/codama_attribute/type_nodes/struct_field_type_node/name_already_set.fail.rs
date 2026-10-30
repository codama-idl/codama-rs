use codama_macros::codama;

#[codama(type = field("age", integer(u8), "years"))]
pub struct Test;

#[codama(type = field(name = "age", type = integer(u8), name = "years"))]
pub struct TestExplicit;

fn main() {}
