use codama::CodamaAccount;
use solana_pubkey::Pubkey;


#[derive(CodamaAccount)]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
