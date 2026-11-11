use codama::CodamaAccount;

#[derive(CodamaAccount)]
pub struct Counter {
    pub count: u64,
}
