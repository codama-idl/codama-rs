use codama_macros::codama;

#[codama(seed(identifier = "missing_field"))]
pub struct TestWithNoFields;

#[codama(seed(identifier = "missing_field"))]
pub struct TestWithFields {
    field1: u8,
    field2: u32,
}

fn main() {}
