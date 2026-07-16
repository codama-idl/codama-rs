use super::Membership;

#[codama(type = struct(field("len", number(u8)), field("value", fixed_size(string, 25))))]
pub type BoundedString25 = PodString<25>;

#[derive(CodamaAccount)]
#[codama(seed(type = string(utf8), value = "person_pda"))]
#[codama(seed(name = "wallet", type = public_key))]
#[codama(seed(name = "name", type = string(utf8)))]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub membership: Membership,
    #[codama(type = public_key)]
    pub wallet: [u8; 32],
    pub bio: BoundedString25,
}
