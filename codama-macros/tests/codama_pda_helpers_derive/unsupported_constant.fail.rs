use codama_macros::{CodamaPda, CodamaPdaHelpers};

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(type = string(base64), value = "cHJlZml4"))]
pub struct Counter;

fn main() {}
