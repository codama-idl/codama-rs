use codama_macros::codama;

#[codama(type = pre_offset(string, 4, relative))]
pub struct ImplicitTest;

#[codama(type = pre_offset(type = string, offset = 4, strategy = absolute))]
pub struct ExplicitTest;

#[codama(type = pre_offset(string, -4, padded))]
pub struct NegativeOffsetTest;

fn main() {}
