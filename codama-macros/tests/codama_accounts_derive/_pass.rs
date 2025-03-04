use codama_macros::CodamaAccounts;

#[derive(CodamaAccounts)]
pub enum TokenAccounts {
    Token {},
    Mint {},
}

fn main() {}
