use codama_macros::codama;

#[codama(type = integer(u32))]
#[codama(type = integer(u32, be))]
#[codama(type = integer(u32, le))]
#[codama(type = integer(le, u32))]
#[codama(type = integer(format = u32, endian = le))]
#[codama(type = integer(endian = le, format = u32))]
#[codama(type = integer(
    u64,
    display = amount(decimals = 9, unit = "SOL")
))]
pub struct Test;

fn main() {}
