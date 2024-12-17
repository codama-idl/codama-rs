use codama::{codama, CodamaType};

#[derive(CodamaType)]
pub struct StructTest {
    #[codama(type = boolean)]
    pub public_field: u8,
    #[codama(type = boolean)]
    private_field: u8,
}

#[derive(CodamaType)]
pub struct TupleTest(#[codama(type = boolean)] u8);

#[derive(CodamaType)]
pub enum EnumTest {
    Struct {
        #[codama(type = boolean)]
        field: u8,
    },
    Tuple(#[codama(type = boolean)] u8),
}

fn main() {}
