use codama_macros::codama;

#[codama(type = link(42))]
pub struct Test;

#[codama(type = link(identifier = 42))]
pub struct TestExplicit;

fn main() {}
