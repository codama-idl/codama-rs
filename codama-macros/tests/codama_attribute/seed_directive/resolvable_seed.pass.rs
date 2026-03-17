use codama_macros::codama;

// Resolvable directives in seed type and value positions should compile.
#[codama(seed(type = foo::custom_type, value = bar::custom_value))]
pub struct Test;

fn main() {}
