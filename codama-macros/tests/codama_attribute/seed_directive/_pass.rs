use codama_macros::codama;

#[codama(seed(name = "authority", type = public_key))]
pub struct TestWithDefinedVariableSeed;

#[codama(seed(type = string(utf8), value = "counter"))]
pub struct TestWithDefinedConstantSeed;

#[codama(seed(name = "authority"))]
pub struct TestWithLinkedSeed {
    authority: String,
}

fn main() {}
