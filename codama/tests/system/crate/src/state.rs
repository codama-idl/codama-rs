use codama::{CodamaAccount, CodamaType};
use solana_pubkey::Pubkey;

#[derive(CodamaAccount)]
pub struct Nonce {
    pub version: NonceVersion,
    pub state: NonceState,
    pub authority: Pubkey,
    pub blockhash: Pubkey,
    pub lamports_per_signature: u64,
}

#[derive(CodamaType)]
#[repr(u32)]
pub enum NonceVersion {
    Legacy,
    Current,
}

#[derive(CodamaType)]
#[repr(u32)]
pub enum NonceState {
    Uninitialized,
    Initialized,
}
