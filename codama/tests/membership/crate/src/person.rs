use super::Membership;

#[derive(CodamaType)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub membership: Membership,
}
