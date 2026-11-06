use codama_macros::codama;

#[codama(type = fixed_point(integer(u64), scale = 6, unit = "USDC"))]
pub struct ImplicitTest;

#[codama(type = fixed_point(number = integer(u64), scale = 9, unit = "SOL"))]
pub struct ExplicitTest;

#[codama(type = fixed_point(integer(u64), scale = 6))]
pub struct ScaleOnlyTest;

#[codama(type = fixed_point(integer(u128), scale = 64, base = 2))]
pub struct BaseTest;

#[codama(type = fixed_point(integer(u64), scale = 9, unit = "SOL", display = unit("wSOL")))]
pub struct DisplayTest;

#[codama(type = fixed_point(fixed_size(integer(u64), 8), scale = 6))]
pub struct NestedNumberTest;

#[codama(field("price", fixed_point(integer(u64), scale = 6, unit = "USDC")))]
pub struct FieldTest;

fn main() {}
