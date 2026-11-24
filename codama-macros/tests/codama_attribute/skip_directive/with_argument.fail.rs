use codama_macros::CodamaErrors;

#[derive(CodamaErrors)]
pub enum Test {
    #[codama(skip = false)]
    NotSkippedAfterAll,
    #[codama(skip(always))]
    Skipped,
}

fn main() {}
