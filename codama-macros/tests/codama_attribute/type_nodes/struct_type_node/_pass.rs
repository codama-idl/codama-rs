use codama_macros::codama;

#[codama(type = struct)]
pub struct EmptyTest;

#[codama(type = struct())]
pub struct EmptyTestWithBraces;

#[codama(type = struct(field("age", number(u32))))]
pub struct OneFieldTest;

#[codama(type = struct(field("age", number(u32)), field("name", string(utf8))))]
pub struct MultipleFieldTest;

#[codama(type = struct[field("age", number(u32)), field("name", string(utf8))])]
pub struct MultipleFieldTestWithBrackets;

fn main() {}
