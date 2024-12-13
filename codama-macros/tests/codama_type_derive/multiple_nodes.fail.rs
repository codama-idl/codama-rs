use codama::{codama, CodamaType};

#[derive(CodamaType)]
#[codama(node(boolean_type))]
#[derive(PartialEq)]
#[codama(node(public_key_type))]
#[derive(Debug)]
#[codama(node(number_type(u32)))]
pub struct StructTest;

#[derive(CodamaType)]
#[codama(node(boolean_type))]
#[codama(node(public_key_type))]
pub enum EnumTest {}

#[derive(CodamaType)]
pub struct StructWithFieldsTest {
    #[codama(node(boolean_type))]
    #[codama(node(public_key_type))]
    pub field: u32,
}

#[derive(CodamaType)]
pub enum EnumWithVariantTest {
    #[codama(node(boolean_type))]
    #[codama(node(public_key_type))]
    Variant,
}

#[derive(CodamaType)]
pub enum EnumWithVariantFieldsTest {
    Variant {
        #[codama(node(boolean_type))]
        #[codama(node(public_key_type))]
        field: u32,
    },
}

fn main() {}
