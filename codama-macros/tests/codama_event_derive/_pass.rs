use codama_macros::CodamaEvent;

#[derive(CodamaEvent)]
pub struct StructTest;

#[derive(CodamaEvent)]
pub enum EnumTest {}

fn main() {}
