use codama_macros::codama;

#[codama(seed(type = number(u8), value = 42, name = "amount"))]
pub struct TestWithName;

#[codama(seed(type = invalid_type, value = 42))]
pub struct TestWithInvalidType;

#[codama(seed(type = number(u8), value = invalid_value))]
pub struct TestWithInvalidValue;

fn main() {}
