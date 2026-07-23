use codama_macros::codama;

#[codama(export)]
pub type Test = u64;

#[codama(export())]
pub type TestEmpty = u64;

#[codama(export(CodamaType, CodamaAccount))]
pub type TestMultiple = u64;

fn main() {}
