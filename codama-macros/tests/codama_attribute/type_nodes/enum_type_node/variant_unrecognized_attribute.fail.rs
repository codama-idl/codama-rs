use codama_macros::codama;

#[codama(type = enum(variant("a", foo = 42)))]
pub struct Test;

fn main() {}
