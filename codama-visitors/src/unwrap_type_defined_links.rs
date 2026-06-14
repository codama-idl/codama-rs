use codama_nodes::{CamelCaseString, ProgramNode, RootNode, TypeNode};
use codama_visitors_core::{fold_program, fold_type_node, LinkableDictionary, TransformVisitor};
use std::collections::HashSet;

/// Replaces selected `definedTypeLinkNode`s with the *body* of the defined type
/// they point at -- a single level of inlining that leaves the defined types
/// themselves in place.
///
/// The Rust counterpart of `@codama/visitors`' `unwrapTypeDefinedLinksVisitor`.
/// Selection is by defined-type name (bare `"type"` or program-qualified
/// `"program.type"`). Unlike [`unwrap_defined_types`](crate::unwrap_defined_types)
/// it neither recurses into the inlined body nor removes the defined types -- it
/// only rewrites the matching links.
///
/// ```
/// use codama_nodes::{
///     AccountNode, DefinedTypeLinkNode, DefinedTypeNode, NestedTypeNodeTrait, NumberTypeNode,
///     ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, TypeNode, U8,
/// };
/// use codama_visitors::unwrap_type_defined_links;
///
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.defined_types.push(DefinedTypeNode::new("foo", NumberTypeNode::le(U8)));
/// program.accounts.push(AccountNode::new(
///     "acc",
///     StructTypeNode::new(vec![StructFieldTypeNode::new(
///         "y",
///         DefinedTypeLinkNode { name: "foo".into(), program: None },
///     )]),
/// ));
///
/// let root = unwrap_type_defined_links(RootNode::new(program), ["foo"]);
/// // `foo` is kept (not removed) ...
/// assert_eq!(root.program.defined_types.len(), 1);
/// // ... but the account's link to it was replaced by foo's number body.
/// let data: &StructTypeNode = root.program.accounts[0].data.get_nested_type_node();
/// assert!(matches!(&*data.fields[0].r#type, TypeNode::Number(_)));
/// ```
pub fn unwrap_type_defined_links(
    root: RootNode,
    names: impl IntoIterator<Item = impl Into<String>>,
) -> RootNode {
    let names = names.into_iter().map(|n| normalize(&n.into())).collect();
    let linkables = LinkableDictionary::from_root(&root);
    let mut visitor = UnwrapTypeDefinedLinks {
        linkables: &linkables,
        names,
        current_program: None,
    };
    visitor.visit_root(root)
}

struct UnwrapTypeDefinedLinks<'a> {
    linkables: &'a LinkableDictionary,
    names: HashSet<String>,
    current_program: Option<String>,
}

impl UnwrapTypeDefinedLinks<'_> {
    fn should_inline(&self, name: &str) -> bool {
        let name = CamelCaseString::new(name).to_string();
        self.names.contains(&name)
            || self
                .current_program
                .as_ref()
                .is_some_and(|p| self.names.contains(&format!("{p}.{name}")))
    }
}

impl TransformVisitor for UnwrapTypeDefinedLinks<'_> {
    fn visit_program(&mut self, node: ProgramNode) -> ProgramNode {
        self.current_program = Some(node.name.to_string());
        let result = fold_program(self, node);
        self.current_program = None;
        result
    }

    fn visit_type_node(&mut self, node: TypeNode) -> TypeNode {
        let node = fold_type_node(self, node);
        if !matches!(node, TypeNode::Link(_)) {
            return node;
        }

        // Resolve the matching link to its body without holding a borrow on `node`.
        let body: Option<TypeNode> = {
            let TypeNode::Link(link) = &node else {
                unreachable!()
            };
            if self.should_inline(link.name.as_ref()) {
                self.linkables
                    .get_defined_type(link, self.current_program.as_deref())
                    .map(|defined_type| (*defined_type.r#type).clone())
            } else {
                None
            }
        };

        // Single level only: the inlined body is not re-visited.
        body.unwrap_or(node)
    }
}

fn normalize(selector: &str) -> String {
    match selector.split_once('.') {
        Some((program, name)) => format!(
            "{}.{}",
            CamelCaseString::new(program).as_ref(),
            CamelCaseString::new(name).as_ref()
        ),
        None => CamelCaseString::new(selector).to_string(),
    }
}
