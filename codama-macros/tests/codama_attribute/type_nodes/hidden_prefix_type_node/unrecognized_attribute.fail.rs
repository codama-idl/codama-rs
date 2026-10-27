use codama_macros::codama;

#[codama(type = hidden_prefix(foo = 42))]
pub struct Test;

fn main() {}
