use codama_macros::CodamaErrors;

#[derive(CodamaErrors)]
pub enum Test {
    #[codama(skip)]
    Skipped,
    #[codama(skip())]
    AlsoSkipped,
    Kept,
}

fn main() {}
