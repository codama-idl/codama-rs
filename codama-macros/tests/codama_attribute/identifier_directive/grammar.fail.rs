use codama_macros::codama;

#[codama(identifier = "my-type")]
pub struct Test;

#[codama(identifier = "1st")]
pub struct TestWithLeadingDigit;

#[codama(identifier = "")]
pub struct TestWithEmptyString;

fn main() {}
