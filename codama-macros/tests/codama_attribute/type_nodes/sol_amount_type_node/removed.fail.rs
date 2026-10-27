use codama_macros::codama;

#[codama(type = sol_amount(number(u64)))]
pub struct Test;

#[codama(type = struct(field("lamports", sol_amount(number(u64)))))]
pub struct NestedTest;

#[codama(field("lamports", sol_amount(number = number(u64))))]
pub struct FieldTest;

fn main() {}
