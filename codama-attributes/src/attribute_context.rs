use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum AttributeContext<'a> {
    File(&'a syn::File),
    Item(&'a syn::Item),
    Variant(&'a syn::Variant),
    Field(&'a syn::Field),
    ItemConst(&'a syn::ItemConst),
    ImplItemConst(&'a syn::ImplItemConst),
}
