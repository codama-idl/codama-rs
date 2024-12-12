use codama::{codama, CodamaType};

#[derive(CodamaType)]
pub struct StructTest {
    #[codama(node())]
    pub public_field: u8,
    #[codama(node())]
    private_field: u8,
}

#[derive(CodamaType)]
pub struct TupleTest(#[codama(node())] u8);

#[derive(CodamaType)]
pub enum EnumTest {
    Struct {
        #[codama(node())]
        field: u8,
    },
    Tuple(#[codama(node())] u8),
}

fn main() {}
