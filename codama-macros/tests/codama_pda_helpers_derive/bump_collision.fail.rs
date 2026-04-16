use codama_macros::{CodamaPda, CodamaPdaHelpers};

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(type = string(utf8), value = "counter"))]
#[codama(seed(name = "bump", type = number(u8)))]
pub struct Counter;

fn main() {}
