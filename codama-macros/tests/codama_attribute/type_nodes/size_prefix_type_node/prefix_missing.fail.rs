use codama_macros::codama;

#[codama(type = size_prefix(string))]
pub struct Test;

#[codama(type = size_prefix(type = string))]
pub struct TestExplicit;

fn main() {}
