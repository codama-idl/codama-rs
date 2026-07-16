use codama_macros::codama;

#[codama(type = link(42))]
pub struct Test;

#[codama(type = link(name = 42))]
pub struct TestExplicit;

fn main() {}
