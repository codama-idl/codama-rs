use codama_macros::{CodamaAccount, CodamaPda, CodamaPdaHelpers};
use solana_address::Address;

#[derive(CodamaAccount, CodamaPdaHelpers)]
#[codama(seed(type = string(utf8), value = "admin_config"))]
pub struct AdminConfig;

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(type = string(utf8), value = "vault"))]
#[codama(seed(name = "authority", type = public_key))]
#[codama(seed(name = "tokenProgram", type = public_key))]
pub struct Vault;

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(name = "wallet", type = public_key))]
#[codama(seed(name = "token_program", type = public_key))]
#[codama(seed(name = "mint", type = public_key))]
pub struct AssociatedTokenPda;

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(type = string(utf8), value = "prefix"))]
#[codama(seed(name = "authority", type = public_key))]
#[codama(seed(type = string(utf8), value = "suffix"))]
pub struct MixedPda;

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(name = "wallet", type = public_key))]
#[codama(seed(name = "wallet", type = public_key))]
pub struct RepeatedWalletPda;

#[derive(CodamaAccount, CodamaPdaHelpers)]
#[codama(seed(type = string(utf8), value = "whitelist"))]
#[codama(seed(name = "authority"))]
pub struct Whitelist {
    authority: [u8; 32],
}

#[derive(CodamaPda, CodamaPdaHelpers)]
#[codama(seed(type = number(u16, le), value = 4660))]
#[codama(seed(type = number(i16, be), value = -2))]
#[codama(seed(name = "authority", type = public_key))]
struct NumberConstantPda;

/// Simulates a framework seed type (like pinocchio::Seed)
struct Seed<'a>(&'a [u8]);
impl<'a> From<&'a [u8]> for Seed<'a> {
    fn from(s: &'a [u8]) -> Self {
        Seed(s)
    }
}

fn main() {
    let admin_config = AdminConfig::seeds();
    let _: &[&[u8]] = &admin_config;
    assert_eq!(admin_config[0], b"admin_config");

    let authority = [1u8; 32];
    let wallet = [9u8; 32];
    let token_program = [2u8; 32];
    let mint = [3u8; 32];
    let bump = [255u8];
    let program_id = Address::new_from_array([7u8; 32]);

    let vault = Vault::seeds(&authority, &token_program);
    let _: &[&[u8]] = &vault;
    assert_eq!(vault[0], b"vault");
    assert_eq!(vault[1], authority.as_ref());
    assert_eq!(vault[2], token_program.as_ref());

    let vault_with_bump = Vault::seeds_with_bump(&authority, &token_program, &bump);
    let _: &[&[u8]] = &vault_with_bump;
    assert_eq!(vault_with_bump[3], bump.as_ref());

    let raw = Vault::signer_seeds::<&[u8]>(&authority, &token_program, &bump);
    assert_eq!(raw[0], b"vault");
    assert_eq!(raw[3], bump.as_ref());

    let seeds = Vault::signer_seeds::<Seed>(&authority, &token_program, &bump);
    assert_eq!(seeds[0].0, b"vault");

    let seeds: [Seed; 4] = Vault::signer_seeds(&authority, &token_program, &bump);
    assert_eq!(seeds[0].0, b"vault");

    let raw = AdminConfig::signer_seeds::<&[u8]>(&bump);
    assert_eq!(raw[0], b"admin_config");
    assert_eq!(raw[1], bump.as_ref());

    let (admin_config_address, admin_config_bump) = AdminConfig::find_program_address(&program_id);
    assert_eq!(
        AdminConfig::derive_address(Some(admin_config_bump), &program_id),
        Address::derive_address(&AdminConfig::seeds(), Some(admin_config_bump), &program_id)
    );
    assert_eq!(
        AdminConfig::create_program_address(admin_config_bump, &program_id).unwrap(),
        Address::create_program_address(
            &AdminConfig::seeds_with_bump(&[admin_config_bump]),
            &program_id
        )
        .unwrap()
    );
    assert_eq!(
        (admin_config_address, admin_config_bump),
        Address::find_program_address(&AdminConfig::seeds(), &program_id)
    );
    assert_eq!(
        AdminConfig::try_find_program_address(&program_id),
        Address::try_find_program_address(&AdminConfig::seeds(), &program_id)
    );
    assert_eq!(
        AdminConfig::derive_program_address(&program_id),
        Address::derive_program_address(&AdminConfig::seeds(), &program_id)
    );

    let ata = AssociatedTokenPda::seeds(&wallet, &token_program, &mint);
    let _: &[&[u8]] = &ata;
    assert_eq!(
        ata,
        [wallet.as_ref(), token_program.as_ref(), mint.as_ref()]
    );

    let (ata_address, ata_bump) =
        AssociatedTokenPda::find_program_address(&wallet, &token_program, &mint, &program_id);
    assert_eq!(
        AssociatedTokenPda::derive_address(&wallet, &token_program, &mint, Some(ata_bump), &program_id),
        Address::derive_address(
            &AssociatedTokenPda::seeds(&wallet, &token_program, &mint),
            Some(ata_bump),
            &program_id
        )
    );
    assert_eq!(
        AssociatedTokenPda::create_program_address(&wallet, &token_program, &mint, ata_bump, &program_id)
            .unwrap(),
        Address::create_program_address(
            &AssociatedTokenPda::seeds_with_bump(&wallet, &token_program, &mint, &[ata_bump]),
            &program_id
        )
        .unwrap()
    );
    assert_eq!(
        (ata_address, ata_bump),
        Address::find_program_address(
            &AssociatedTokenPda::seeds(&wallet, &token_program, &mint),
            &program_id
        )
    );
    assert_eq!(
        AssociatedTokenPda::try_find_program_address(&wallet, &token_program, &mint, &program_id),
        Address::try_find_program_address(
            &AssociatedTokenPda::seeds(&wallet, &token_program, &mint),
            &program_id
        )
    );
    assert_eq!(
        AssociatedTokenPda::derive_program_address(&wallet, &token_program, &mint, &program_id),
        Address::derive_program_address(
            &AssociatedTokenPda::seeds(&wallet, &token_program, &mint),
            &program_id
        )
    );

    let ata_signer =
        AssociatedTokenPda::signer_seeds::<Seed>(&wallet, &token_program, &mint, &bump);
    assert_eq!(ata_signer[0].0, wallet.as_ref());
    assert_eq!(ata_signer[3].0, bump.as_ref());

    let mixed = MixedPda::seeds(&authority);
    assert_eq!(mixed[0], b"prefix");
    assert_eq!(mixed[1], authority.as_ref());
    assert_eq!(mixed[2], b"suffix");

    let repeated = RepeatedWalletPda::seeds(&wallet);
    assert_eq!(repeated, [wallet.as_ref(), wallet.as_ref()]);

    let whitelist = Whitelist::seeds(&authority);
    assert_eq!(whitelist[0], b"whitelist");
    assert_eq!(whitelist[1], authority.as_ref());

    let number_seeds = NumberConstantPda::seeds(&authority);
    assert_eq!(number_seeds[0], &[0x34, 0x12]);
    assert_eq!(number_seeds[1], &[0xff, 0xfe]);
    assert_eq!(number_seeds[2], authority.as_ref());
}
