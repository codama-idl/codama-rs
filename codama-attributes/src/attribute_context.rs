use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum AttributeContext<'a> {
    File(&'a syn::File),
    Item(&'a syn::Item),
    Variant(&'a syn::Variant),
    Field(&'a syn::Field),
    ImplItem(&'a syn::ImplItem),
}

impl<'a> AttributeContext<'a> {
    pub fn get_fields(&self) -> Option<&'a syn::Fields> {
        match self {
            AttributeContext::Item(syn::Item::Struct(syn::ItemStruct { fields, .. })) => {
                Some(fields)
            }
            AttributeContext::Variant(syn::Variant { fields, .. }) => Some(fields),
            _ => None,
        }
    }

    pub fn get_named_fields(&self) -> Option<&'a syn::FieldsNamed> {
        match self.get_fields() {
            Some(syn::Fields::Named(fields)) => Some(fields),
            _ => None,
        }
    }
}
