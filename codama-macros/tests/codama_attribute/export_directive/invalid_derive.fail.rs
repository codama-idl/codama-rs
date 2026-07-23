use codama_macros::codama;

#[codama(export(NotADerive))]
pub type Test = u64;

fn main() {}
