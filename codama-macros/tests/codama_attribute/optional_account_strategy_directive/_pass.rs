use codama_macros::{codama, CodamaInstruction, CodamaInstructions};

#[derive(CodamaInstruction)]
#[codama(optional_account_strategy = omitted)]
pub struct SingleInstruction;

#[derive(CodamaInstructions)]
pub enum InstructionSet {
    #[codama(optional_account_strategy = omitted)]
    Create,
}

fn main() {}
