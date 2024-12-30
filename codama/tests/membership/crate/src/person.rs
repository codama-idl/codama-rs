use super::Membership;

#[derive(CodamaAccount)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub membership: Membership,
    #[codama(type = public_key)]
    pub wallet: [u8; 32],
}
