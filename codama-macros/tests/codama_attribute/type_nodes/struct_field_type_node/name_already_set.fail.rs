use codama_macros::codama;

#[codama(type = field("age", integer(u8), "years"))]
pub struct Test;

#[codama(type = field(identifier = "age", type = integer(u8), identifier = "years"))]
pub struct TestExplicit;

fn main() {}
