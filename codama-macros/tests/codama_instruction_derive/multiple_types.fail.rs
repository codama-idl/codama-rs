use codama_macros::{codama, CodamaInstruction};

#[derive(CodamaInstruction)]
#[codama(type = boolean)]
#[derive(PartialEq)]
#[codama(type = public_key)]
#[derive(Debug)]
#[codama(type = number(u32))]
pub struct StructTest {
    #[codama(type = boolean)]
    #[codama(type = public_key)]
    pub field: u32,
}

#[derive(CodamaInstruction)]
#[codama(type = boolean)]
#[codama(type = public_key)]
pub enum EnumTest {
    #[codama(type = boolean)]
    #[codama(type = public_key)]
    Variant {
        #[codama(type = boolean)]
        #[codama(type = public_key)]
        field: u32,
    },
}

fn main() {}
