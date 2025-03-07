use codama_macros::codama;

#[codama(default_value = 0)]
#[codama(default_value = 42)]
#[codama(default_value = -42)]
#[codama(default_value = 1.5)]
#[codama(default_value = -1.5)]
pub struct Test;

fn main() {}
