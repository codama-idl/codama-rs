use codama_macros::codama;

#[codama(argument("age", number(u8), default_value = 42))]
#[codama(argument(after, "name", string(utf8)))]
#[codama(argument(after, "wallet", public_key, default_value = payer))]
pub struct Test;

fn main() {}
