use codama_macros::codama;

#[codama(default_value = pda("fee", [data("config.fees[0]")]))]
pub struct TestWithNestedDataShortcut;

fn main() {}
