use codama_macros::{CodamaPda, CodamaPdaHelpers};

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(type = string(utf8), value = "counter"))]
#[codama(seed(name = "token-program", type = public_key))]
#[codama(seed(name = "token_program", type = public_key))]
pub struct Counter;

fn main() {}
