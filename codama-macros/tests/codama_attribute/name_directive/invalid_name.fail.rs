use codama_macros::codama;

#[codama(name = 42)]
pub struct Test;

#[codama(name = invalid)]
pub struct TestWithPath;

#[codama(name = invalid(1, 2, 3))]
pub struct TestWithList;

fn main() {}
