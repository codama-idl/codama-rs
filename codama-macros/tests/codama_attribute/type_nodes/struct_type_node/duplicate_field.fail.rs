use codama_macros::codama;

#[codama(type = struct(field("owner", public_key), field("owner", integer(u64))))]
pub struct Test;

fn main() {}
