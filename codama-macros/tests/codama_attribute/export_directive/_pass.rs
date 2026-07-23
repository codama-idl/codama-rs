use codama_macros::codama;

#[codama(export(CodamaType))]
pub type Lamports = u64;

#[codama(export(CodamaAccount))]
pub type MyAccount = u64;

#[codama(export(CodamaInstruction))]
pub type MyInstruction = u64;

fn main() {}
