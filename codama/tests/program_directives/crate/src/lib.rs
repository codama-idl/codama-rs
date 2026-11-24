solana_program::declare_id!("MainProg1111111111111111111111111111111111111");

#[codama(program(identifier = "external", address = "ExtProg111111111111111111111111111111111111"))]
mod external;

mod other;

#[derive(CodamaAccount)]
pub struct Config {
    pub owner: Pubkey,
}
