use codama_macros::CodamaInstructions;

#[derive(CodamaInstructions)]
pub enum Instructions {
    #[codama(optional_account_strategy = invalid)]
    Create,
}

fn main() {}
