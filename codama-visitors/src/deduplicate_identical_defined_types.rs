use codama_nodes::{DefinedTypeNode, NodeTrait, RootNode};
use codama_visitors_core::{delete_nodes, RemoveDocs, TransformVisitor};
use std::collections::HashMap;

/// Removes defined types that are duplicated — same name and identical structure
/// (ignoring docs) — across the programs of an IDL, keeping the first occurrence.
///
/// The Rust counterpart of `@codama/visitors`' `deduplicateIdenticalDefinedTypesVisitor`.
/// Root-coupled: it inspects every program's defined types, then deletes the
/// redundant copies via [`delete_nodes`](codama_visitors_core::delete_nodes).
/// Structural equality is decided by a docs-stripped JSON hash.
///
/// ```
/// use codama_nodes::{DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode, U8};
/// use codama_visitors::deduplicate_identical_defined_types;
///
/// let mut a = ProgramNode::new("a", "Myprogram1111111111111111111111111111111111");
/// a.defined_types.push(DefinedTypeNode::new("foo", NumberTypeNode::le(U8)));
/// let mut b = ProgramNode::new("b", "Bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
/// b.defined_types.push(DefinedTypeNode::new("foo", NumberTypeNode::le(U8)));
/// let mut root = RootNode::new(a);
/// root.additional_programs.push(b);
///
/// let root = deduplicate_identical_defined_types(root);
/// assert!(root.additional_programs[0].defined_types.is_empty()); // b's `foo` removed
/// ```
pub fn deduplicate_identical_defined_types(root: RootNode) -> RootNode {
    let programs = std::iter::once(&root.program).chain(root.additional_programs.iter());

    // Group every defined type by name, preserving program order within a group.
    let mut groups: HashMap<String, Vec<(String, DefinedTypeNode)>> = HashMap::new();
    for program in programs {
        for defined_type in &program.defined_types {
            groups
                .entry(defined_type.name.to_string())
                .or_default()
                .push((program.name.to_string(), defined_type.clone()));
        }
    }

    let mut delete_selectors: Vec<String> = Vec::new();
    for group in groups.values() {
        if group.len() <= 1 {
            continue;
        }
        let hashes: Vec<String> = group.iter().map(|(_, dt)| structural_hash(dt)).collect();
        let all_identical = hashes.iter().all(|h| h == &hashes[0]);
        if all_identical {
            // Keep the first; delete every later duplicate.
            for (program_name, defined_type) in &group[1..] {
                delete_selectors.push(format!(
                    "[programNode]{program_name}.[definedTypeNode]{}",
                    defined_type.name.as_ref()
                ));
            }
        }
    }

    if delete_selectors.is_empty() {
        return root;
    }
    delete_nodes(delete_selectors).visit_root(root)
}

/// A docs-insensitive structural fingerprint of a defined type.
fn structural_hash(defined_type: &DefinedTypeNode) -> String {
    let mut remover = RemoveDocs;
    remover
        .visit_defined_type(defined_type.clone())
        .to_json()
        .unwrap_or_default()
}
