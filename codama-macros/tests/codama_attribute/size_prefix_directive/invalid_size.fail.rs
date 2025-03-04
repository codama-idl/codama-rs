use codama_macros::codama;

#[codama(size_prefix = string)]
pub struct TestWithNonNumberTypeNode;

#[codama(size_prefix = invalid(1, 2, 3))]
pub struct TestWithInvalidValue;

fn main() {}
