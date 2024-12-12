use codama::codama;

#[codama(node())]
pub struct StructTest;

#[codama(node())]
pub enum EnumTest {}

#[codama(node())]
pub trait TraitTest {}

#[codama(node())]
pub type TypeTest = ();

#[codama(node())]
pub mod mod_test {}

#[codama(node())]
pub fn fn_test() {}

#[codama(node())]
pub const CONST_TEST: () = ();

#[codama(node())]
pub static STATIC_TEST: () = ();

#[codama(node())]
pub union UnionTest {
    _field: (),
}

#[codama(node())]
pub extern "C" fn extern_fn_test() {}

fn main() {}
