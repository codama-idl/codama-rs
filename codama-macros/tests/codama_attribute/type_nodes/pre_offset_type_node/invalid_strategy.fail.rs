use codama_macros::codama;

#[codama(type = pre_offset(string, 4, strategy = nonsense))]
pub struct Test;

fn main() {}
