use codama::{codama, CodamaAccount};

#[derive(CodamaAccount)]
#[codama(node(boolean_type))]
#[derive(PartialEq)]
#[codama(node(public_key_type))]
#[derive(Debug)]
#[codama(node(number_type(u32)))]
pub struct StructTest {
    #[codama(node(boolean_type))]
    #[codama(node(public_key_type))]
    pub field: u32,
}

#[derive(CodamaAccount)]
#[codama(node(boolean_type))]
#[codama(node(public_key_type))]
pub enum EnumTest {
    #[codama(node(boolean_type))]
    #[codama(node(public_key_type))]
    Variant {
        #[codama(node(boolean_type))]
        #[codama(node(public_key_type))]
        field: u32,
    },
}

fn main() {}
