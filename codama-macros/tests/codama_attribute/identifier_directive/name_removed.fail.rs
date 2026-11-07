use codama_macros::{codama, CodamaAccount, CodamaInstruction, CodamaPda};

#[codama(name = "my_type")]
pub struct Test;

#[derive(CodamaAccount)]
#[codama(field(name = "owner", public_key))]
pub struct MyAccount;

#[derive(CodamaInstruction)]
#[codama(account(name = "payer", signer))]
#[codama(remaining_accounts(name = "signers", signer))]
pub struct MyInstruction;

#[derive(CodamaPda)]
#[codama(program(name = "token", address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))]
#[codama(seed(name = "owner", type = public_key))]
pub struct MyPda;

fn main() {}
