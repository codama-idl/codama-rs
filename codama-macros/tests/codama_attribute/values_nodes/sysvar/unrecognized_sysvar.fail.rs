use codama_macros::codama;

#[codama(default_value = sysvar("banana"))]
pub struct Test;

fn main() {}
