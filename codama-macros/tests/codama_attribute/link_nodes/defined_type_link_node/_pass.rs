use codama_macros::codama;

#[codama(type = link("customString"))]
pub struct ImplicitTest;

#[codama(type = link(name = "customString"))]
pub struct ExplicitTest;

#[codama(type = option(link("customString")))]
pub struct NestedTest;

#[codama(type = option(link("customString"), fixed))]
pub struct NestedFixedTest;

fn main() {}
