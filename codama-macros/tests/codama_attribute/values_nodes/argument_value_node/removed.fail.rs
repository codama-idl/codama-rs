use codama_macros::codama;

#[codama(default_value = argument("amount"))]
pub struct Test;

#[codama(default_value = pda("token", [argument("amount")]))]
pub struct SeedShortcutTest;

#[codama(default_value = pda("token", [seed("amount", argument("amount"))]))]
pub struct SeedValueTest;

fn main() {}
