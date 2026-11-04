use codama_macros::codama;

#[codama(type = link("customString", "anotherString"))]
pub struct Test;

#[codama(type = link(identifier = "customString", identifier = "anotherString"))]
pub struct TestExplicit;

fn main() {}
