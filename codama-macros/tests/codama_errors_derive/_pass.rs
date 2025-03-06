use codama_macros::CodamaErrors;

#[derive(CodamaErrors)]
pub enum MyProgramErrors {
    WrongArgument,
    WrongAccount,
}

fn main() {}
