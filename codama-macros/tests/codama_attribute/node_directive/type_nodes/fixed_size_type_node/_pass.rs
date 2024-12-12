use codama::codama;

#[codama(node(fixed_size_type(boolean_type, 42)))]
#[codama(node(fixed_size_type(type = boolean_type, size = 42)))]
pub struct Test;

fn main() {}
