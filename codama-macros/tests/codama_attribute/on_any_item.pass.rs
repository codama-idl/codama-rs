use codama::codama;

#[codama(type = boolean)]
pub struct StructTest;

#[codama(type = boolean)]
pub enum EnumTest {}

#[codama(type = boolean)]
pub trait TraitTest {}

#[codama(type = boolean)]
pub type TypeTest = ();

#[codama(type = boolean)]
pub mod mod_test {}

#[codama(type = boolean)]
pub fn fn_test() {}

#[codama(type = boolean)]
pub const CONST_TEST: () = ();

#[codama(type = boolean)]
pub static STATIC_TEST: () = ();

#[codama(type = boolean)]
pub union UnionTest {
    _field: (),
}

#[codama(type = boolean)]
pub extern "C" fn extern_fn_test() {}

fn main() {}
