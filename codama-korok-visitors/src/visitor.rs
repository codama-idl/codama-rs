use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_koroks::{
    ConstKorok, CrateKorok, EnumKorok, EnumVariantKorok, FieldKorok, FileModuleKorok,
    ImplItemKorok, ImplKorok, ItemKorok, KorokMut, ModuleKorok, RootKorok, StructKorok,
    TypeAliasKorok, UnsupportedImplItemKorok, UnsupportedItemKorok,
};

/// Visits the koroks of a tree, one method per kind of korok, like `syn::visit_mut::VisitMut`.
///
/// Each method defaults to walking the children of its korok with the matching `walk_*` function.
/// A method that handles a korok itself calls that function to keep walking:
///
/// ```
/// use codama_errors::CodamaResult;
/// use codama_korok_visitors::{walk_struct, KorokVisitor};
/// use codama_koroks::StructKorok;
///
/// struct CountFields(usize);
///
/// impl KorokVisitor for CountFields {
///     fn visit_struct(&mut self, korok: &mut StructKorok) -> CodamaResult<()> {
///         self.0 += korok.fields.len();
///         walk_struct(self, korok)
///     }
/// }
/// ```
pub trait KorokVisitor {
    fn visit_root(&mut self, korok: &mut RootKorok) -> CodamaResult<()> {
        walk_root(self, korok)
    }

    fn visit_crate(&mut self, korok: &mut CrateKorok) -> CodamaResult<()> {
        walk_crate(self, korok)
    }

    fn visit_item(&mut self, korok: &mut ItemKorok) -> CodamaResult<()> {
        walk_item(self, korok)
    }

    fn visit_file_module(&mut self, korok: &mut FileModuleKorok) -> CodamaResult<()> {
        walk_file_module(self, korok)
    }

    fn visit_module(&mut self, korok: &mut ModuleKorok) -> CodamaResult<()> {
        walk_module(self, korok)
    }

    fn visit_struct(&mut self, korok: &mut StructKorok) -> CodamaResult<()> {
        walk_struct(self, korok)
    }

    fn visit_enum(&mut self, korok: &mut EnumKorok) -> CodamaResult<()> {
        walk_enum(self, korok)
    }

    fn visit_enum_variant(&mut self, korok: &mut EnumVariantKorok) -> CodamaResult<()> {
        walk_enum_variant(self, korok)
    }

    fn visit_field(&mut self, korok: &mut FieldKorok) -> CodamaResult<()> {
        walk_field(self, korok)
    }

    fn visit_impl(&mut self, korok: &mut ImplKorok) -> CodamaResult<()> {
        walk_impl(self, korok)
    }

    fn visit_impl_item(&mut self, korok: &mut ImplItemKorok) -> CodamaResult<()> {
        walk_impl_item(self, korok)
    }

    fn visit_const(&mut self, korok: &mut ConstKorok) -> CodamaResult<()> {
        walk_const(self, korok)
    }

    fn visit_type_alias(&mut self, korok: &mut TypeAliasKorok) -> CodamaResult<()> {
        walk_type_alias(self, korok)
    }

    fn visit_unsupported_item(&mut self, korok: &mut UnsupportedItemKorok) -> CodamaResult<()> {
        walk_unsupported_item(self, korok)
    }

    fn visit_unsupported_impl_item(
        &mut self,
        korok: &mut UnsupportedImplItemKorok,
    ) -> CodamaResult<()> {
        walk_unsupported_impl_item(self, korok)
    }
}

/// Visits every crate of the root.
pub fn walk_root<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut RootKorok,
) -> CodamaResult<()> {
    korok
        .crates
        .iter_mut()
        .map(|korok| visitor.visit_crate(korok))
        .collect_and_combine_errors()?;
    Ok(())
}

/// Visits every item of the crate.
pub fn walk_crate<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut CrateKorok,
) -> CodamaResult<()> {
    walk_items(visitor, &mut korok.items)
}

/// Visits the item as the korok of its kind.
pub fn walk_item<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut ItemKorok,
) -> CodamaResult<()> {
    match korok {
        ItemKorok::FileModule(korok) => visitor.visit_file_module(korok),
        ItemKorok::Module(korok) => visitor.visit_module(korok),
        ItemKorok::Struct(korok) => visitor.visit_struct(korok),
        ItemKorok::Enum(korok) => visitor.visit_enum(korok),
        ItemKorok::Impl(korok) => visitor.visit_impl(korok),
        ItemKorok::Const(korok) => visitor.visit_const(korok),
        ItemKorok::TypeAlias(korok) => visitor.visit_type_alias(korok),
        ItemKorok::Unsupported(korok) => visitor.visit_unsupported_item(korok),
    }
}

/// Visits every item of the module declared in its own file.
pub fn walk_file_module<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut FileModuleKorok,
) -> CodamaResult<()> {
    walk_items(visitor, &mut korok.items)
}

/// Visits every item of the inline module.
pub fn walk_module<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut ModuleKorok,
) -> CodamaResult<()> {
    walk_items(visitor, &mut korok.items)
}

/// Visits every field of the struct.
pub fn walk_struct<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut StructKorok,
) -> CodamaResult<()> {
    walk_fields(visitor, &mut korok.fields)
}

/// Visits every variant of the enum.
pub fn walk_enum<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut EnumKorok,
) -> CodamaResult<()> {
    korok
        .variants
        .iter_mut()
        .map(|korok| visitor.visit_enum_variant(korok))
        .collect_and_combine_errors()?;
    Ok(())
}

/// Visits every field of the variant.
pub fn walk_enum_variant<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut EnumVariantKorok,
) -> CodamaResult<()> {
    walk_fields(visitor, &mut korok.fields)
}

/// A field has no children.
pub fn walk_field<V: KorokVisitor + ?Sized>(
    _visitor: &mut V,
    _korok: &mut FieldKorok,
) -> CodamaResult<()> {
    Ok(())
}

/// Visits every item of the `impl` block.
pub fn walk_impl<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut ImplKorok,
) -> CodamaResult<()> {
    korok
        .items
        .iter_mut()
        .map(|korok| visitor.visit_impl_item(korok))
        .collect_and_combine_errors()?;
    Ok(())
}

/// Visits the `impl` item as the korok of its kind.
pub fn walk_impl_item<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut ImplItemKorok,
) -> CodamaResult<()> {
    match korok {
        ImplItemKorok::Const(korok) => visitor.visit_const(korok),
        ImplItemKorok::Unsupported(korok) => visitor.visit_unsupported_impl_item(korok),
    }
}

/// A constant has no children.
pub fn walk_const<V: KorokVisitor + ?Sized>(
    _visitor: &mut V,
    _korok: &mut ConstKorok,
) -> CodamaResult<()> {
    Ok(())
}

/// A type alias has no children.
pub fn walk_type_alias<V: KorokVisitor + ?Sized>(
    _visitor: &mut V,
    _korok: &mut TypeAliasKorok,
) -> CodamaResult<()> {
    Ok(())
}

/// An unsupported item has no children.
pub fn walk_unsupported_item<V: KorokVisitor + ?Sized>(
    _visitor: &mut V,
    _korok: &mut UnsupportedItemKorok,
) -> CodamaResult<()> {
    Ok(())
}

/// An unsupported `impl` item has no children.
pub fn walk_unsupported_impl_item<V: KorokVisitor + ?Sized>(
    _visitor: &mut V,
    _korok: &mut UnsupportedImplItemKorok,
) -> CodamaResult<()> {
    Ok(())
}

/// Walks the children of any korok, like the `walk_*` function of its kind.
pub fn walk_korok_mut<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    korok: &mut KorokMut,
) -> CodamaResult<()> {
    match korok {
        KorokMut::Root(korok) => walk_root(visitor, korok),
        KorokMut::Crate(korok) => walk_crate(visitor, korok),
        KorokMut::Item(korok) => walk_item(visitor, korok),
        KorokMut::FileModule(korok) => walk_file_module(visitor, korok),
        KorokMut::Module(korok) => walk_module(visitor, korok),
        KorokMut::Struct(korok) => walk_struct(visitor, korok),
        KorokMut::Enum(korok) => walk_enum(visitor, korok),
        KorokMut::EnumVariant(korok) => walk_enum_variant(visitor, korok),
        KorokMut::Field(korok) => walk_field(visitor, korok),
        KorokMut::Impl(korok) => walk_impl(visitor, korok),
        KorokMut::ImplItem(korok) => walk_impl_item(visitor, korok),
        KorokMut::Const(korok) => walk_const(visitor, korok),
        KorokMut::TypeAlias(korok) => walk_type_alias(visitor, korok),
        KorokMut::UnsupportedItem(korok) => walk_unsupported_item(visitor, korok),
        KorokMut::UnsupportedImplItem(korok) => walk_unsupported_impl_item(visitor, korok),
    }
}

fn walk_items<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    items: &mut [ItemKorok],
) -> CodamaResult<()> {
    items
        .iter_mut()
        .map(|korok| visitor.visit_item(korok))
        .collect_and_combine_errors()?;
    Ok(())
}

fn walk_fields<V: KorokVisitor + ?Sized>(
    visitor: &mut V,
    fields: &mut [FieldKorok],
) -> CodamaResult<()> {
    fields
        .iter_mut()
        .map(|korok| visitor.visit_field(korok))
        .collect_and_combine_errors()?;
    Ok(())
}
