use codama::CodamaInstructions;
use solana_pubkey::Pubkey;

#[derive(CodamaInstructions)]
#[repr(u32)]
pub enum SystemInstruction {
    #[codama(account(name = "payer", signer, writable, default_value = payer))]
    #[codama(account(name = "new_account", signer, writable))]
    CreateAccount {
        lamports: u64,
        space: u64,
        program_address: Pubkey,
    },

    #[codama(account(name = "account", signer, writable))]
    Assign { program_address: Pubkey },

    #[codama(account(name = "source", signer, writable))]
    #[codama(account(name = "destination", writable))]
    TransferSol { amount: u64 },

    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "new_account", writable))]
    #[codama(account(name = "base_account", signer))]
    CreateAccountWithSeed {
        base: Pubkey,
        #[codama(size_prefix = number(u64))]
        seed: String,
        amount: u64,
        space: u64,
        program_address: Pubkey,
    },

    #[codama(account(name = "nonce_account", writable))]
    #[codama(account(name = "recent_blockhashes_sysvar", default_value = sysvar("recent_blockhashes")))]
    #[codama(account(name = "nonce_authority", signer))]
    AdvanceNonceAccount,

    #[codama(account(name = "nonce_account", writable))]
    #[codama(account(name = "recipient_account", writable))]
    #[codama(account(name = "recent_blockhashes_sysvar", default_value = sysvar("recent_blockhashes")))]
    #[codama(account(name = "rent_sysvar", default_value = sysvar("rent")))]
    #[codama(account(name = "nonce_authority", signer))]
    WithdrawNonceAccount { withdraw_amount: u64 },

    #[codama(account(name = "nonce_account", writable))]
    #[codama(account(name = "recent_blockhashes_sysvar", default_value = sysvar("recent_blockhashes")))]
    #[codama(account(name = "rent_sysvar", default_value = sysvar("rent")))]
    InitializeNonceAccount { nonce_authority: Pubkey },

    #[codama(account(name = "nonce_account", writable))]
    #[codama(account(name = "nonce_authority", signer))]
    AuthorizeNonceAccount { new_nonce_authority: Pubkey },

    #[codama(account(name = "new_account", signer, writable))]
    Allocate { space: u64 },

    #[codama(account(name = "new_account", writable))]
    #[codama(account(name = "base_account", signer))]
    AllocateWithSeed {
        base: Pubkey,
        #[codama(size_prefix = number(u64))]
        seed: String,
        space: u64,
        program_address: Pubkey,
    },

    #[codama(account(name = "account", writable))]
    #[codama(account(name = "base_account", signer))]
    AssignWithSeed {
        base: Pubkey,
        #[codama(size_prefix = number(u64))]
        seed: String,
        program_address: Pubkey,
    },

    #[codama(account(name = "source", writable))]
    #[codama(account(name = "base_account", signer))]
    #[codama(account(name = "destination", writable))]
    TransferSolWithSeed {
        amount: u64,
        #[codama(size_prefix = number(u64))]
        from_seed: String,
        from_owner: Pubkey,
    },

    #[codama(account(name = "nonce_account", writable))]
    UpgradeNonceAccount,
}
