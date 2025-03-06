use codama_macros::codama;

#[codama(error(42, "my message"))]
#[codama(error("my message", 42))]
#[codama(error(code = 42, message = "my message"))]
#[codama(error(message = "my message", code = 42))]
#[codama(error("my message", code = 42))]
pub struct Test;

fn main() {}
