use codama_macros::codama;

#[codama(default_value = data("amount"))]
pub struct Test;

#[codama(default_value = data("config.fees[0]"))]
pub struct TestWithPath;

fn main() {}
