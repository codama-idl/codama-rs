use codama_macros::codama;

// Resolvable directives in field type and value positions should compile.
#[codama(field("age", foo::custom_type, default_value = bar::custom_value))]
pub struct Test {
    name: String,
}

fn main() {}
