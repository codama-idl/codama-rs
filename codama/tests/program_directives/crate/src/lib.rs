solana_program::declare_id!("MainProg1111111111111111111111111111111111111");

#[codama(program(identifier = "external", address = "ExtProg111111111111111111111111111111111111"))]
mod external;

// The declaration wins over the directive at the top of the file.
#[codama(program(identifier = "declared", address = "Dec1aredProg111111111111111111111111111111"))]
mod both;

mod other;

#[derive(CodamaAccount)]
pub struct Config {
    pub owner: Pubkey,
}
