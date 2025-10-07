use codama_macros::codama;

#[codama(discriminator(field = "discriminator", size = 100))]
pub struct TestWithFieldAndSize;

#[codama(discriminator(size = 100, field = "discriminator"))]
pub struct TestWithSizeAndField;

#[codama(discriminator(field = "discriminator", bytes = [1, 2, 3, 4]))]
pub struct TestWithFieldAndBytes;

#[codama(discriminator(bytes = [1, 2, 3, 4], field = "discriminator"))]
pub struct TestWithBytesAndField;

#[codama(discriminator(size = 100, bytes = [1, 2, 3, 4]))]
pub struct TestWithSizeAndBytes;

#[codama(discriminator(bytes = [1, 2, 3, 4], size = 100))]
pub struct TestWithBytesAndSize;

#[codama(discriminator(bytes = [1, 2, 3, 4], field = "discriminator", size = 100))]
pub struct TestWithBytesFieldAndSize;

fn main() {}
