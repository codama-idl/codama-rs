solana_program::declare_id!("Vault11111111111111111111111111111111111111");

#[derive(CodamaAccounts)]
pub enum MyAccounts {
    #[codama(pda = "vault")]
    #[codama(seed(identifier = "authority", type = public_key))]
    Vault { authority: Pubkey },
    #[codama(pda = "vault")]
    #[codama(seed(identifier = "authority", type = public_key))]
    LegacyVault { authority: Pubkey },
}
