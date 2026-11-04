use codama_macros::codama;

#[codama(seed(identifier = "authority", type = public_key))]
pub struct TestWithDefinedVariableSeed;

#[codama(seed(type = string(utf8), value = "counter"))]
pub struct TestWithDefinedConstantSeed;

#[codama(seed(identifier = "authority"))]
pub struct TestWithLinkedSeed {
    authority: String,
}

fn main() {}
