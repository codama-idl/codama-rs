use codama::codama;

#[codama(fixed_size = invalid(1, 2, 3))]
pub struct Test;

#[codama(fixed_size = invalid)]
pub struct TestWithExpr;

fn main() {}
