use codama_macros::codama;

#[codama(type = amount(number(u64), 9, "SOL"))]
pub struct Test;

#[codama(type = struct(field("price", amount(number(u64), 6, "USDC"))))]
pub struct NestedTest;

#[codama(field("size", amount(number = number(u64), decimals = 0, unit = "bytes")))]
pub struct FieldTest;

fn main() {}
