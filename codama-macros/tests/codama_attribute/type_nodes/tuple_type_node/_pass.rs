use codama_macros::codama;

#[codama(type = tuple)]
pub struct EmptyTest;

#[codama(type = tuple())]
pub struct EmptyListTest;

#[codama(type = tuple(number(u32)))]
pub struct SingleItemTest;

#[codama(type = tuple(number(u32), string, public_key))]
pub struct MultipleItemsTest;

#[codama(type = tuple(option(number(u8)), tuple(string)))]
pub struct NestedTest;

fn main() {}
