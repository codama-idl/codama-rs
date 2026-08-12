use codama_macros::codama;

#[codama(type = number(u32))]
#[codama(type = number(u32, be))]
#[codama(type = number(u32, le))]
#[codama(type = number(le, u32))]
#[codama(type = number(format = u32, endian = le))]
#[codama(type = number(endian = le, format = u32))]
#[codama(type = number(
    u64,
    display = amount(decimals = 9, unit = "SOL")
))]
#[codama(type = number(i64, display = date_time))]
#[codama(type = number(
    i64,
    display = date_time(ticks_per_second = 1_000)
))]
pub struct Test;

fn main() {}
