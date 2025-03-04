use codama_macros::{codama, CodamaType};

#[derive(CodamaType)]
pub struct StructTest {
    #[codama(type = invalid)]
    pub public_field: u8,
    #[codama(type = invalid)]
    private_field: u8,
}

#[derive(CodamaType)]
pub struct TupleTest(#[codama(type = invalid)] u8);

#[derive(CodamaType)]
pub enum EnumTest {
    Struct {
        #[codama(type = invalid)]
        field: u8,
    },
    Tuple(#[codama(type = invalid)] u8),
}

fn main() {}
