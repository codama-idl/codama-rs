use codama_macros::codama;

#[codama(field("age", number(u8), default_value = 42))]
#[codama(field(after, "name", string(utf8)))]
pub struct Test;

fn main() {}
