//! Visitors that follow links, the way the JavaScript library combines its adapters.

use codama_nodes::{
    DefinedTypeLinkNode, DefinedTypeNode, IntegerTypeNode, NodeRef, ProgramNode, RootNode,
    StructFieldTypeNode, StructTypeNode, U16, U8,
};
use codama_visitors_core::{visit, LinkableDictionary, MergeVisitor, NodeStack, VisitorExt};

fn root() -> RootNode {
    RootNode::new(ProgramNode {
        defined_types: vec![
            DefinedTypeNode::new(
                "header",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("version", IntegerTypeNode::le(U8)),
                    StructFieldTypeNode::new("size", IntegerTypeNode::le(U16)),
                ]),
            ),
            DefinedTypeNode::new(
                "message",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("header", DefinedTypeLinkNode::new("header")),
                    StructFieldTypeNode::new("missing", DefinedTypeLinkNode::new("missing")),
                ]),
            ),
        ],
        ..ProgramNode::new("protocol", "1111")
    })
}

/// Lists the integers that each defined type holds, following the links to other defined types,
/// and reports the links that point to no defined type.
#[test]
fn lists_integers_through_links() {
    let root = root();
    let stack = NodeStack::default();
    let linkables = LinkableDictionary::default();
    let visitor = MergeVisitor::new(|_, values: Vec<Vec<String>>| values.concat())
        .extend(|node, scope| match node {
            NodeRef::IntegerType(_) => vec![stack.to_string()],
            NodeRef::DefinedTypeLink(link) => match linkables.get_path(&stack.get_path()) {
                Some(linked) => stack.visit_path(linked, |linked| scope.visit(linked)),
                None => vec![format!("missing {}", link.identifier)],
            },
            node => scope.next(node),
        })
        .record_node_stack(&stack)
        .record_linkables_on_first_visit(&linkables);

    let header = "[programNode]protocol.[definedTypeNode]header.[structTypeNode]";
    assert_eq!(
        visit(&visitor, &root),
        [
            // The integers of the `header` type itself.
            format!("{header}.[structFieldTypeNode]version.[integerTypeNode]"),
            format!("{header}.[structFieldTypeNode]size.[integerTypeNode]"),
            // The `message` type reaches the same integers through its link, from their own path.
            format!("{header}.[structFieldTypeNode]version.[integerTypeNode]"),
            format!("{header}.[structFieldTypeNode]size.[integerTypeNode]"),
            "missing missing".to_string(),
        ]
    );
    assert!(stack.is_empty());
}
