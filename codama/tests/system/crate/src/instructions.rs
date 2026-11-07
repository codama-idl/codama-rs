use codama::CodamaInstructions;
use solana_pubkey::Pubkey;

#[derive(CodamaInstructions)]
#[repr(u32)]
pub enum SystemInstruction {
    #[codama(account(identifier = "payer", signer, writable, default_value = payer))]
    #[codama(account(identifier = "new_account", signer, writable))]
    CreateAccount {
        lamports: u64,
        space: u64,
        program_address: Pubkey,
    },

    #[codama(account(identifier = "account", signer, writable))]
    Assign { program_address: Pubkey },

    #[codama(display(
        intent = "Transfer SOL",
        interpolated_intent = "Transfer ${data.amount} from ${accounts.source} to ${accounts.destination}"
    ))]
    #[codama(account(identifier = "source", signer, writable, display(label = "From")))]
    #[codama(account(identifier = "destination", writable, display(label = "To")))]
    TransferSol {
        #[codama(display(label = "Amount", amount(decimals = 9, unit = "SOL")))]
        amount: u64,
    },

    #[codama(account(identifier = "payer", signer, writable))]
    #[codama(account(identifier = "new_account", writable))]
    #[codama(account(identifier = "base_account", signer))]
    CreateAccountWithSeed {
        base: Pubkey,
        #[codama(size_prefix = integer(u64))]
        seed: String,
        amount: u64,
        space: u64,
        program_address: Pubkey,
    },

    #[codama(account(identifier = "nonce_account", writable))]
    #[codama(account(identifier = "recent_blockhashes_sysvar", default_value = sysvar("recent_blockhashes")))]
    #[codama(account(identifier = "nonce_authority", signer))]
    AdvanceNonceAccount,

    #[codama(account(identifier = "nonce_account", writable))]
    #[codama(account(identifier = "recipient_account", writable))]
    #[codama(account(identifier = "recent_blockhashes_sysvar", default_value = sysvar("recent_blockhashes")))]
    #[codama(account(identifier = "rent_sysvar", default_value = sysvar("rent")))]
    #[codama(account(identifier = "nonce_authority", signer))]
    WithdrawNonceAccount { withdraw_amount: u64 },

    #[codama(account(identifier = "nonce_account", writable))]
    #[codama(account(identifier = "recent_blockhashes_sysvar", default_value = sysvar("recent_blockhashes")))]
    #[codama(account(identifier = "rent_sysvar", default_value = sysvar("rent")))]
    InitializeNonceAccount { nonce_authority: Pubkey },

    #[codama(account(identifier = "nonce_account", writable))]
    #[codama(account(identifier = "nonce_authority", signer))]
    AuthorizeNonceAccount { new_nonce_authority: Pubkey },

    #[codama(account(identifier = "new_account", signer, writable))]
    Allocate { space: u64 },

    #[codama(account(identifier = "new_account", writable))]
    #[codama(account(identifier = "base_account", signer))]
    AllocateWithSeed {
        base: Pubkey,
        #[codama(size_prefix = integer(u64))]
        seed: String,
        space: u64,
        program_address: Pubkey,
    },

    #[codama(account(identifier = "account", writable))]
    #[codama(account(identifier = "base_account", signer))]
    AssignWithSeed {
        base: Pubkey,
        #[codama(size_prefix = integer(u64))]
        seed: String,
        program_address: Pubkey,
    },

    #[codama(account(identifier = "source", writable))]
    #[codama(account(identifier = "base_account", signer))]
    #[codama(account(identifier = "destination", writable))]
    TransferSolWithSeed {
        amount: u64,
        #[codama(size_prefix = integer(u64))]
        from_seed: String,
        from_owner: Pubkey,
    },

    #[codama(account(identifier = "nonce_account", writable))]
    UpgradeNonceAccount,
}
