use codama_macros::codama;

#[codama(discriminator)]
pub struct EmptyTest;

#[codama(discriminator())]
pub struct EmptyTest2;

#[codama(discriminator(offset = 42))]
pub struct TestWithOffsetOnly;

#[codama(discriminator(encoding = "utf8"))]
pub struct TestWithEncodingOnly;

fn main() {}
