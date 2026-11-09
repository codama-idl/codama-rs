use codama_macros::codama;

#[codama(seed(identifier = "amount", value = 42))]
pub struct TestWithValue;

#[codama(seed(identifier = 42, type = integer(u8)))]
pub struct TestWithInvalidName;

#[codama(seed(identifier = "1st", type = integer(u8)))]
pub struct TestWithInvalidIdentifier;

#[codama(seed(identifier = "authority", type = invalid_type))]
pub struct TestWithInvalidType;

fn main() {}
