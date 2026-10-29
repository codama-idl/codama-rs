use codama_macros::codama;

#[codama(type = float(f64, display = amount(decimals = 2, unit = "USD")))]
pub struct Test;

fn main() {}
