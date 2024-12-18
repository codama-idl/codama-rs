use codama::codama;

#[codama(type = string(utf8, base64))]
pub struct Test;

#[codama(type = string(encoding = utf8, encoding = base64))]
pub struct TestExplicit;

fn main() {}
