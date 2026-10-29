use codama_macros::codama;

#[codama(display(amount))]
pub struct BareTest;

#[codama(display(amount(unit = "SOL")))]
pub struct UnitOnlyTest;

fn main() {}
