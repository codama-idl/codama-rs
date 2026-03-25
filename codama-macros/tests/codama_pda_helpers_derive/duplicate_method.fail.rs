use codama_macros::{CodamaAccount, CodamaPdaHelpers};

#[derive(CodamaAccount, CodamaPdaHelpers)]
#[codama(seed(type = string(utf8), value = "counter"))]
pub struct Counter;

impl Counter {
    pub fn seeds() -> [&'static [u8]; 0] {
        []
    }
}

fn main() {}
