use codama_macros::codama;

#[codama(type = tuple)]
pub struct EmptyTest;

#[codama(type = tuple())]
pub struct EmptyListTest;

#[codama(type = tuple(integer(u32)))]
pub struct SingleItemTest;

#[codama(type = tuple(integer(u32), string, public_key))]
pub struct MultipleItemsTest;

#[codama(type = tuple(option(integer(u8)), tuple(string)))]
pub struct NestedTest;

fn main() {}
