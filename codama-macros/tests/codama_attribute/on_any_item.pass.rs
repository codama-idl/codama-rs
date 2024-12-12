use codama::codama;

#[codama(node(boolean_type))]
pub struct StructTest;

#[codama(node(boolean_type))]
pub enum EnumTest {}

#[codama(node(boolean_type))]
pub trait TraitTest {}

#[codama(node(boolean_type))]
pub type TypeTest = ();

#[codama(node(boolean_type))]
pub mod mod_test {}

#[codama(node(boolean_type))]
pub fn fn_test() {}

#[codama(node(boolean_type))]
pub const CONST_TEST: () = ();

#[codama(node(boolean_type))]
pub static STATIC_TEST: () = ();

#[codama(node(boolean_type))]
pub union UnionTest {
    _field: (),
}

#[codama(node(boolean_type))]
pub extern "C" fn extern_fn_test() {}

fn main() {}
