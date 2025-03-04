use codama_macros::CodamaInstructions;

#[derive(CodamaInstructions)]
pub enum TokenInstructions {
    InitializeMint {},
    InitializeToken {},
    Transfer {},
}

fn main() {}
