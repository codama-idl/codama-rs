use codama_macros::codama;

#[codama(type = hidden_suffix(string, constant(number(u8), 1)))]
pub struct SingleConstantTest;

#[codama(type = hidden_suffix(string, constant(number(u8), 1), constant(number(u8), 2)))]
pub struct MultipleConstantsTest;

fn main() {}
