use codama_macros::codama;

#[codama(type = hidden_suffix(foo = 42))]
pub struct Test;

fn main() {}
