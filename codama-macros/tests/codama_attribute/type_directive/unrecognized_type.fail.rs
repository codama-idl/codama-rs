use codama_macros::codama;

#[codama(type = unrecognized_type(foo = 42))]
pub struct Test;

fn main() {}
