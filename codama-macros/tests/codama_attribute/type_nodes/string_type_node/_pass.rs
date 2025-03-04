use codama_macros::codama;

#[codama(type = string)]
#[codama(type = string())]
#[codama(type = string(utf8))]
#[codama(type = string(base58))]
#[codama(type = string(encoding = base58))]
pub struct Test;

fn main() {}
