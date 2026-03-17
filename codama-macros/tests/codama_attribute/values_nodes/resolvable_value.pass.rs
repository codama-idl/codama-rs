use codama_macros::codama;

// Resolvable directives in value positions should compile without error.
#[codama(default_value = wellknown::ata(account("owner"), account("tokenProgram"), account("mint")))]
pub struct Test;

fn main() {}
