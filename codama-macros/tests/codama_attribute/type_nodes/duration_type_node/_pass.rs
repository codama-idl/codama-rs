use codama_macros::codama;

#[codama(type = duration(integer(u32)))]
pub struct ImplicitTest;

#[codama(type = duration(number = integer(u32)))]
pub struct ExplicitTest;

#[codama(type = duration(integer(u64), ticks_per_second = 1000))]
pub struct TicksPerSecondTest;

#[codama(type = duration(fixed_size(integer(u64), 8)))]
pub struct NestedNumberTest;

#[codama(field("timeout", duration(integer(u64), ticks_per_second = 1000)))]
pub struct FieldTest;

fn main() {}
