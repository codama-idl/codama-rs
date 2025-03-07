use codama_macros::codama;

#[codama(default_value = sysvar("rent", "clock"))]
pub struct Test;

fn main() {}
