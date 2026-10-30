use codama_macros::codama;

#[codama(type = struct(integer(u32), boolean, field("age", integer(u8)), 3 * 4))]
pub struct Test;

fn main() {}
