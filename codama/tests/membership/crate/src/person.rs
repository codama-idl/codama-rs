use super::Membership;

#[derive(CodamaAccount)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub membership: Membership,
    #[codama(type = public_key)]
    pub wallet: [u8; 32],
}

#[derive(CodamaPda)]
#[codama(seed(type = string(utf8), value = "person_pda"))]
#[codama(seed(name = "authority", type = public_key))]
#[codama(seed(name = "name", type = string(utf8)))]
pub struct PersonPda;
