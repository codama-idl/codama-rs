use codama_macros::codama;

#[codama(type = field("owner", public_key, default_value = payer))]
pub struct Test;

#[codama(type = field("bump", integer(u8), value = account_bump("vault")))]
pub struct TestOmitted;

fn main() {}
