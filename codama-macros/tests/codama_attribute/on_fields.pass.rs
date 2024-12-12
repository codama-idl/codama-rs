use codama::{codama, CodamaType};

#[derive(CodamaType)]
pub struct StructTest {
    #[codama(node(boolean_type))]
    pub public_field: u8,
    #[codama(node(boolean_type))]
    private_field: u8,
}

#[derive(CodamaType)]
pub struct TupleTest(#[codama(node(boolean_type))] u8);

#[derive(CodamaType)]
pub enum EnumTest {
    Struct {
        #[codama(node(boolean_type))]
        field: u8,
    },
    Tuple(#[codama(node(boolean_type))] u8),
}

fn main() {}
