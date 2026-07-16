use codama_macros::codama;

#[codama(type = link("customString", "anotherString"))]
pub struct Test;

#[codama(type = link(name = "customString", name = "anotherString"))]
pub struct TestExplicit;

fn main() {}
