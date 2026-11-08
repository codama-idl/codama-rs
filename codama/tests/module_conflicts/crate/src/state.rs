use codama::CodamaType;

mod v1 {
    #[derive(CodamaType)]
    pub struct Config {
        pub amount: u8,
    }
}

mod v2 {
    #[derive(CodamaType)]
    pub struct Config {
        pub amount: u16,
    }
}
