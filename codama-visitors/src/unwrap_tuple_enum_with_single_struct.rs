use crate::{get_defined_type_histogram, unwrap_defined_types_named};
use codama_nodes::{
    EnumStructVariantTypeNode, EnumVariantTypeNode, NestedTypeNodeTrait, ProgramNode, RootNode,
    StructTypeNode, TypeNode,
};
use codama_visitors_core::{
    fold_enum_variant_type_node, fold_program, LinkableDictionary, TransformVisitor,
};
use std::collections::HashSet;

/// Converts enum *tuple* variants that wrap a single struct into enum *struct*
/// variants, then removes any defined type that became unused as a result.
///
/// The Rust counterpart of `@codama/visitors`' `unwrapTupleEnumWithSingleStructVisitor`
/// (with a `'*'` selection). Composes [`LinkableDictionary`] (to resolve a
/// single-item `definedTypeLinkNode` to its struct), a custom enum-variant
/// rewriter, [`get_defined_type_histogram`](crate::get_defined_type_histogram),
/// and [`unwrap_defined_types_named`](crate::unwrap_defined_types_named).
pub fn unwrap_tuple_enum_with_single_struct(root: RootNode) -> RootNode {
    let linkables = LinkableDictionary::from_root(&root);
    let mut rewriter = Rewriter {
        linkables: &linkables,
        current_program: None,
        inlined_types: HashSet::new(),
    };
    let root = rewriter.visit_root(root);
    let inlined_types = std::mem::take(&mut rewriter.inlined_types);
    drop(rewriter);

    if inlined_types.is_empty() {
        return root;
    }

    // Remove the inlined defined types that are no longer referenced anywhere.
    let histogram = get_defined_type_histogram(&root);
    let orphaned: Vec<String> = inlined_types
        .into_iter()
        .filter(|qualified| {
            histogram
                .get(qualified)
                .is_none_or(|entry| entry.total == 0)
        })
        .collect();

    if orphaned.is_empty() {
        return root;
    }
    unwrap_defined_types_named(root, orphaned)
}

struct Rewriter<'a> {
    linkables: &'a LinkableDictionary,
    current_program: Option<String>,
    /// Program-qualified names (`"program.type"`) inlined from a link.
    inlined_types: HashSet<String>,
}

impl TransformVisitor for Rewriter<'_> {
    fn visit_program(&mut self, node: ProgramNode) -> ProgramNode {
        self.current_program = Some(node.name.to_string());
        let result = fold_program(self, node);
        self.current_program = None;
        result
    }

    fn visit_enum_variant_type_node(&mut self, node: EnumVariantTypeNode) -> EnumVariantTypeNode {
        let node = fold_enum_variant_type_node(self, node);
        let EnumVariantTypeNode::Tuple(_) = &node else {
            return node;
        };

        // Find the single struct item (directly, or behind a defined-type link).
        let linkables = self.linkables;
        let current = self.current_program.clone();
        let found: Option<(StructTypeNode, Option<String>)> = {
            let EnumVariantTypeNode::Tuple(variant) = &node else {
                unreachable!()
            };
            let tuple = variant.tuple.get_nested_type_node();
            if tuple.items.len() != 1 {
                None
            } else {
                match &tuple.items[0] {
                    TypeNode::Struct(s) => Some((s.clone(), None)),
                    TypeNode::Link(link) => linkables
                        .get_defined_type(link, current.as_deref())
                        .and_then(|defined_type| match &*defined_type.r#type {
                            TypeNode::Struct(s) => {
                                let qualified = format!(
                                    "{}.{}",
                                    current.as_deref().unwrap_or_default(),
                                    link.name.as_ref()
                                );
                                Some((s.clone(), Some(qualified)))
                            }
                            _ => None,
                        }),
                    _ => None,
                }
            }
        };

        let Some((struct_node, inlined)) = found else {
            return node;
        };
        if let Some(qualified) = inlined {
            self.inlined_types.insert(qualified);
        }

        let EnumVariantTypeNode::Tuple(variant) = node else {
            unreachable!()
        };
        // Preserve the wrapper chain, swapping the inner tuple for the struct.
        let nested_struct = variant.tuple.map_nested_type_node(|_| struct_node);
        EnumVariantTypeNode::Struct(EnumStructVariantTypeNode {
            name: variant.name,
            discriminator: variant.discriminator,
            r#struct: nested_struct,
        })
    }
}
