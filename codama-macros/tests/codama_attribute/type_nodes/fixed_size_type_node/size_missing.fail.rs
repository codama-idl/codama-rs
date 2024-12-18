use codama::codama;

#[codama(type = fixed_size(boolean))]
pub struct Test;

#[codama(type = fixed_size(type = boolean))]
pub struct TestExplicit;

fn main() {}
