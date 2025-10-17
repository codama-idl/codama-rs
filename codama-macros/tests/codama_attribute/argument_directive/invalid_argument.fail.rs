use codama_macros::codama;

#[codama(argument())]
pub struct EmptyTest;

#[codama(argument(number(u8)))]
pub struct NameMissingTest;

#[codama(argument("age"))]
pub struct TypeMissingTest;

fn main() {}
