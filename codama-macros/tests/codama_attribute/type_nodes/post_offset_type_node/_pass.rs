use codama_macros::codama;

#[codama(type = post_offset(string, 4, relative))]
pub struct ImplicitTest;

#[codama(type = post_offset(type = string, offset = 4, strategy = absolute))]
pub struct ExplicitTest;

#[codama(type = post_offset(string, 0, pre_offset))]
pub struct PreOffsetStrategyTest;

#[codama(type = post_offset(pre_offset(string, 4, relative), 0, pre_offset))]
pub struct WrappingPreOffsetTest;

fn main() {}
