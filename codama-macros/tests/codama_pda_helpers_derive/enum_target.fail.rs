use codama_macros::{CodamaPda, CodamaPdaHelpers};

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(type = string(utf8), value = "counter"))]
pub enum Counter {}

fn main() {}
