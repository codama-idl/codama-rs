use codama_macros::{codama, CodamaInstruction};

pub struct AccountMeta;

#[derive(CodamaInstruction)]
pub struct MyInstruction {
    #[codama(account(signer))]
    pub authority: AccountMeta,
    #[codama(account(signer, writable))]
    pub payer: AccountMeta,
    #[codama(account)]
    pub owner: AccountMeta,
    #[codama(account(signer = "either", optional))]
    pub delegate: AccountMeta,
}

#[derive(CodamaInstruction)]
pub struct MyInstructionWithExplicitValues {
    #[codama(account(signer = true, writable = false))]
    pub authority: AccountMeta,
    #[codama(account(signer = true, writable = true))]
    pub payer: AccountMeta,
    #[codama(account(signer = false, writable = false))]
    pub owner: AccountMeta,
    #[codama(account(signer = "either", writable = false, optional = true))]
    pub delegate: AccountMeta,
}

#[codama(account(name = "authority", signer))]
#[codama(account(name = "payer", signer, writable))]
#[codama(account(name = "owner"))]
#[codama(account(name = "delegate", signer = "either", optional))]
pub struct MyInstructionWithoutAccountFields;

fn main() {}
