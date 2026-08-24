use codama::CodamaInstruction;

#[derive(CodamaInstruction)]
#[codama(remaining_accounts(
    argument("signers"),
    signer,
    optional,
    docs = "Expected signers of the memo."
))]
pub struct AddMemo {
    #[codama(type = string(utf8))]
    pub memo: String,
}
