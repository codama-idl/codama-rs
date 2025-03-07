use codama_macros::codama;

#[codama(default_value = sysvar("rent"))]
pub struct Test;

fn main() {}
