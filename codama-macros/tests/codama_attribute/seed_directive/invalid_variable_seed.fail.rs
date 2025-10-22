use codama_macros::codama;

#[codama(seed(name = "amount", value = 42))]
pub struct TestWithValue;

#[codama(seed(name = 42, type = number(u8)))]
pub struct TestWithInvalidName;

#[codama(seed(name = "authority", type = invalid_type))]
pub struct TestWithInvalidType;

fn main() {}
