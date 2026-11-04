use codama_macros::codama;

#[codama(identifier = 42)]
pub struct Test;

#[codama(identifier = invalid)]
pub struct TestWithPath;

#[codama(identifier = invalid(1, 2, 3))]
pub struct TestWithList;

fn main() {}
