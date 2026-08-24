use codama_macros::{codama, CodamaInstruction, CodamaInstructions};

#[derive(CodamaInstruction)]
#[codama(remaining_accounts(argument("signers"), signer, optional))]
pub struct MyInstruction {
    pub amount: u64,
}

#[derive(CodamaInstruction)]
#[codama(remaining_accounts(
    argument(name = "signers"),
    signer = "either",
    writable = false,
    optional = true,
    docs = ["Line 1", "Line 2"],
    display(label = "Signers")
))]
pub struct MyInstructionWithExplicitValues;

#[derive(CodamaInstruction)]
#[codama(remaining_accounts(argument("signers"), signer))]
#[codama(remaining_accounts(argument("extra_accounts"), writable))]
pub struct MyInstructionWithMultipleRemainingAccounts;

#[derive(CodamaInstructions)]
pub enum MyProgramInstructions {
    #[codama(remaining_accounts(argument("signers"), signer, docs = "Additional signers."))]
    Initialize,
    Close,
}

fn main() {}
