use codama::codama;

#[codama(type = invalid)]
pub struct StructTest;

#[codama(type = invalid)]
pub enum EnumTest {}

#[codama(type = invalid)]
pub trait TraitTest {}

#[codama(type = invalid)]
pub type TypeTest = ();

#[codama(type = invalid)]
pub mod mod_test {}

#[codama(type = invalid)]
pub fn fn_test() {}

#[codama(type = invalid)]
pub const CONST_TEST: () = ();

#[codama(type = invalid)]
pub static STATIC_TEST: () = ();

#[codama(type = invalid)]
pub union UnionTest {
    _field: (),
}

#[codama(type = invalid)]
pub extern "C" fn extern_fn_test() {}

fn main() {}
