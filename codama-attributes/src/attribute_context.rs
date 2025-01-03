use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum AttributeContext<'a> {
    File(&'a syn::File),
    Variant(&'a syn::Variant),
    Field(&'a syn::Field),
    ItemStruct(&'a syn::ItemStruct),
    ItemEnum(&'a syn::ItemEnum),
    ItemMod(&'a syn::ItemMod),
    UnsupportedItem(&'a syn::Item),
}
