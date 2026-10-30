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
#[codama(type = integer(u64, unit = "slots"))]
#[codama(type = integer(format = u16, endian = be, unit = "bps"))]
#[codama(type = integer(
    u64,
    unit = "lamports",
    display = amount(decimals = 9, unit = "SOL")
))]
pub struct Test;

fn main() {}
