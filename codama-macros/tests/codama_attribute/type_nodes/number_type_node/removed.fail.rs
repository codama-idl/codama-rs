use codama_macros::codama;

#[codama(type = number(u64))]
pub struct Test;

#[codama(type = struct(field("len", number(u8))))]
pub struct NestedTest;

#[codama(field("len", number(u8)))]
pub struct FieldTest;

fn main() {}
