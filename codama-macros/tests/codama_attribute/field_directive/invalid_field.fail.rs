use codama_macros::codama;

#[codama(field())]
pub struct EmptyTest;

#[codama(field(number(u8)))]
pub struct NameMissingTest;

#[codama(field("age"))]
pub struct TypeMissingTest;

fn main() {}
