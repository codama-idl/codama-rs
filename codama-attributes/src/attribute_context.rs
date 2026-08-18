use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum AttributeContext<'a> {
    /// The root of a crate, i.e. the inner attributes of its `lib.rs`/`main.rs`.
    ///
    /// Distinct from [`AttributeContext::File`] (a file-module) because only the
    /// crate root carries the primary program's default identity. Constructed
    /// explicitly (never via `From`) so `From<&syn::File>` keeps mapping to
    /// [`AttributeContext::File`].
    #[from(skip)]
    Crate(&'a syn::File),
    /// A file-module, i.e. the inner attributes of a `mod foo;` living in its
    /// own `foo.rs`.
    File(&'a syn::File),
    /// A top-level item, such as a `struct`, `enum` or inline `mod { .. }`.
    Item(&'a syn::Item),
    /// An enum variant.
    Variant(&'a syn::Variant),
    /// A field of a struct or an enum variant.
    Field(&'a syn::Field),
    /// An item within an `impl` block.
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
