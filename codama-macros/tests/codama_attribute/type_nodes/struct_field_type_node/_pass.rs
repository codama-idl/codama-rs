use codama_macros::codama;

#[codama(type = field("age", number(u32)))]
pub struct ImplicitTest;

#[codama(type = field(name = "age", type = number(u32)))]
pub struct ExplicitTest;

#[codama(type = field("age", number(u32), default_value = 42))]
pub struct TestWithDefaultValue;

#[codama(type = field("age", number(u32), default_value = 42, default_value_omitted))]
pub struct TestWithDefaultValueStrategy;

fn main() {}
