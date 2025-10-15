use codama_macros::codama;

#[codama(type = struct(number(u32), boolean, field("age", number(u8)), 3 * 4))]
pub struct Test;

fn main() {}
