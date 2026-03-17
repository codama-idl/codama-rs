use codama_macros::codama;

// Resolvable directives nested inside an account directive should compile without error.
#[codama(account(name = "vault", writable, default_value = wellknown::ata(account("owner"))))]
pub struct Test;

fn main() {}
