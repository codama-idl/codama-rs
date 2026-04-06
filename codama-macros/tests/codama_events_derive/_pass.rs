use codama_macros::CodamaEvents;

#[derive(CodamaEvents)]
pub enum MyProgramEvents {
    Transfer {},
    Burn {},
}

fn main() {}
