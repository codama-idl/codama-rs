use codama_macros::codama;

#[codama(field("age", integer(u8), default_value = 42))]
#[codama(field(after, "name", string(utf8)))]
#[codama(field("owner", public_key, default_value = payer))]
#[codama(field(after, "bump", integer(u8), value = account_bump("vault")))]
pub struct Test;

fn main() {}
