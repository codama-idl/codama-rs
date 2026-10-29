use codama_macros::codama;

#[codama(type = float(f32))]
#[codama(type = float(f64, be))]
#[codama(type = float(f64, le))]
#[codama(type = float(le, f64))]
#[codama(type = float(format = f64, endian = le))]
#[codama(type = float(endian = le, format = f64))]
#[codama(type = float(f64, unit = "USD"))]
#[codama(type = float(format = f32, endian = be, unit = "%"))]
#[codama(type = float(f64, display = unit("USD")))]
#[codama(type = float(f64, unit = "USD", display = unit("EUR")))]
pub struct Test;

fn main() {}
