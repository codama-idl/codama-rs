use codama_nodes::{
    CamelCaseString, EnumStructVariantTypeNode, EnumTupleVariantTypeNode, EnumTypeNode,
    EnumVariantTypeNode, StructTypeNode,
};
use std::collections::HashMap;

/// Renames the fields of a struct according to `map` (old name -> new name).
/// Fields absent from the map are left untouched. The Rust counterpart of
/// `renameStructNode`.
pub(crate) fn rename_struct_node(
    mut node: StructTypeNode,
    map: &HashMap<CamelCaseString, String>,
) -> StructTypeNode {
    for field in node.fields.iter_mut() {
        if let Some(new_name) = map.get(&field.name) {
            field.name = new_name.clone().into();
        }
    }
    node
}

/// Renames the variants of an enum according to `map`. As upstream, renaming a
/// struct/tuple variant drops its discriminator. The Rust counterpart of
/// `renameEnumNode`.
pub(crate) fn rename_enum_node(
    node: EnumTypeNode,
    map: &HashMap<CamelCaseString, String>,
) -> EnumTypeNode {
    let variants = node
        .variants
        .into_iter()
        .map(|variant| match map.get(variant_name(&variant)) {
            Some(new_name) => rename_variant(variant, new_name),
            None => variant,
        })
        .collect();
    EnumTypeNode { variants, ..node }
}

fn rename_variant(variant: EnumVariantTypeNode, new_name: &str) -> EnumVariantTypeNode {
    match variant {
        EnumVariantTypeNode::Struct(v) => {
            EnumStructVariantTypeNode::new(new_name, v.r#struct).into()
        }
        EnumVariantTypeNode::Tuple(v) => EnumTupleVariantTypeNode::new(new_name, v.tuple).into(),
        EnumVariantTypeNode::Empty(_) => {
            codama_nodes::EnumEmptyVariantTypeNode::new(new_name).into()
        }
    }
}

fn variant_name(variant: &EnumVariantTypeNode) -> &CamelCaseString {
    match variant {
        EnumVariantTypeNode::Empty(v) => &v.name,
        EnumVariantTypeNode::Struct(v) => &v.name,
        EnumVariantTypeNode::Tuple(v) => &v.name,
    }
}

/// Builds a node selector that constrains the last path segment to `kind`.
///
/// A bare key (`"myAccount"`) becomes `"[accountNode]myAccount"`, while a
/// program-scoped key (`"myProgram.myAccount"`) becomes
/// `"myProgram.[accountNode]myAccount"` -- mirroring the upstream
/// `['[accountNode]', selector]` conjunction so dotted, program-scoped selectors
/// match the right node.
pub(crate) fn scoped_selector(kind: &str, key: &str) -> String {
    match key.rsplit_once('.') {
        Some((prefix, last)) => format!("{prefix}.[{kind}]{last}"),
        None => format!("[{kind}]{key}"),
    }
}

/// Builds a camelCase-keyed rename map from raw `(old, new)` pairs.
pub(crate) fn rename_map(
    pairs: impl IntoIterator<Item = (String, String)>,
) -> HashMap<CamelCaseString, String> {
    pairs
        .into_iter()
        .map(|(old, new)| (CamelCaseString::new(old), new))
        .collect()
}
