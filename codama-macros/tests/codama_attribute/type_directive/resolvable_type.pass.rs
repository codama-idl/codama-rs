use codama_macros::codama;

// Resolvable directives in type positions should compile without error.
#[codama(type = foo::custom_type)]
pub struct Test;

fn main() {}
