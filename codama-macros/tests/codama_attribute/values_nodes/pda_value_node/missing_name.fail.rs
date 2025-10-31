use codama_macros::codama;

#[codama(default_value = pda)]
pub struct TestEmpty;

#[codama(default_value = pda())]
pub struct TestEmptyWithBraces;

#[codama(default_value = pda(seeds = []))]
pub struct TestWithExplicitSeeds;

#[codama(default_value = pda([]))]
pub struct TestWithImplicitSeeds;

fn main() {}
