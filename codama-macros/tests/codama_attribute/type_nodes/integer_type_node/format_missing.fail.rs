use codama_macros::codama;

#[codama(type = integer(le))]
pub struct Test;

#[codama(type = integer(endian = le))]
pub struct TestExplicit;

fn main() {}
