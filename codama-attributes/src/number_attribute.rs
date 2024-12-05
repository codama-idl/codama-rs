use codama_nodes::{Endian, NumberFormat};

#[derive(Debug, PartialEq)]
pub struct NumberAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub endian: Option<Endian>,
    pub format: Option<NumberFormat>,
}
