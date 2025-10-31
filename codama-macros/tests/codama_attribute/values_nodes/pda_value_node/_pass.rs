use codama_macros::codama;

#[codama(default_value = pda(name = "token"))]
pub struct TestExplicit;

#[codama(default_value = pda("token"))]
pub struct TestImplicit;

#[codama(default_value = pda(name = "token", seeds = []))]
pub struct TestExplicitWithEmptySeeds;

#[codama(default_value = pda("token", []))]
pub struct TestImplicitWithEmptySeeds;

#[codama(default_value = pda(name = "token", seeds = [
  seed(name = "mint", value = account("token_mint")),
  seed(name = "owner", value = account("authority")),
]))]
pub struct TestExplicitWithSeeds;

#[codama(default_value = pda("token", [
  seed("mint", account("token_mint")),
  seed("owner", account("authority")),
]))]
pub struct TestImplicitWithSeeds;

#[codama(default_value = pda("token", [account("mint"), account("owner")]))]
pub struct TestWithShortcutSeeds;

#[codama(default_value = pda("token", [seed("identifier", 42)]))]
pub struct TestWithImplicitSeed;

#[codama(default_value = pda("token", [seed("name", "usdc")]))]
pub struct TestWithImplicitStringSeed;

fn main() {}
