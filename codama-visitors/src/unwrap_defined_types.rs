use codama_nodes::{CamelCaseString, ProgramNode, RootNode, TypeNode};
use codama_visitors_core::{fold_program, fold_type_node, LinkableDictionary, TransformVisitor};
use std::collections::HashSet;

/// Inlines selected defined types at every `definedTypeLinkNode` referencing
/// them, then removes those defined types from the IDL.
///
/// The Rust counterpart of `@codama/visitors`' `unwrapDefinedTypesVisitor` with a
/// `'*'` selection: it inlines *all* defined types. Root-coupled (builds a
/// [`LinkableDictionary`] from the whole tree first), and implemented as a custom
/// [`TransformVisitor`] so the inlined body can be recursively unwrapped.
///
/// ```
/// use codama_nodes::{AccountNode, DefinedTypeLinkNode, DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U8};
/// use codama_visitors::unwrap_defined_types;
///
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.defined_types.push(DefinedTypeNode::new("foo", NumberTypeNode::le(U8)));
/// program.accounts.push(AccountNode::new("acc", StructTypeNode::new(vec![
///     StructFieldTypeNode::new("y", DefinedTypeLinkNode { name: "foo".into(), program: None }),
/// ])));
/// let root = unwrap_defined_types(RootNode::new(program));
/// assert!(root.program.defined_types.is_empty());
/// ```
///
/// Note: a cyclic defined type cannot be fully inlined; the cycle's residual
/// link is left in place.
pub fn unwrap_defined_types(root: RootNode) -> RootNode {
    run(root, Selection::All)
}

/// Like [`unwrap_defined_types`] but only inlines the named defined types
/// (bare `"type"` or program-qualified `"program.type"`).
pub fn unwrap_defined_types_named(
    root: RootNode,
    names: impl IntoIterator<Item = impl Into<String>>,
) -> RootNode {
    let set = names.into_iter().map(|n| normalize(&n.into())).collect();
    run(root, Selection::Only(set))
}

fn run(root: RootNode, selection: Selection) -> RootNode {
    let linkables = LinkableDictionary::from_root(&root);
    let mut visitor = UnwrapDefinedTypes {
        linkables: &linkables,
        selection,
        current_program: None,
        in_progress: Vec::new(),
    };
    visitor.visit_root(root)
}

enum Selection {
    All,
    Only(HashSet<String>),
}

struct UnwrapDefinedTypes<'a> {
    linkables: &'a LinkableDictionary,
    selection: Selection,
    current_program: Option<String>,
    in_progress: Vec<String>,
}

impl UnwrapDefinedTypes<'_> {
    fn should_inline(&self, name: &str) -> bool {
        match &self.selection {
            Selection::All => true,
            Selection::Only(set) => {
                let name = CamelCaseString::new(name).to_string();
                set.contains(&name)
                    || self
                        .current_program
                        .as_ref()
                        .is_some_and(|p| set.contains(&format!("{p}.{name}")))
            }
        }
    }
}

impl TransformVisitor for UnwrapDefinedTypes<'_> {
    fn visit_program(&mut self, mut node: ProgramNode) -> ProgramNode {
        self.current_program = Some(node.name.to_string());
        node.defined_types
            .retain(|defined_type| !self.should_inline(defined_type.name.as_ref()));
        let result = fold_program(self, node);
        self.current_program = None;
        result
    }

    fn visit_type_node(&mut self, node: TypeNode) -> TypeNode {
        let node = fold_type_node(self, node);
        if !matches!(node, TypeNode::Link(_)) {
            return node;
        }

        // Resolve the link (cloning what we need) without holding a borrow on `node`.
        let linkables = self.linkables;
        let resolved: Option<(String, Option<String>, TypeNode)> = {
            let TypeNode::Link(link) = &node else {
                unreachable!()
            };
            if !self.should_inline(link.name.as_ref()) {
                None
            } else {
                linkables
                    .get_defined_type(link, self.current_program.as_deref())
                    .map(|defined_type| {
                        (
                            defined_type.name.to_string(),
                            link.program.as_ref().map(|p| p.name.to_string()),
                            (*defined_type.r#type).clone(),
                        )
                    })
            }
        };

        let Some((name, link_program, body)) = resolved else {
            return node;
        };
        if self.in_progress.iter().any(|n| n == &name) {
            return node; // cyclic: keep the residual link
        }

        let previous_program = self.current_program.clone();
        if link_program.is_some() {
            self.current_program = link_program;
        }
        self.in_progress.push(name);
        let unwrapped = self.visit_type_node(body);
        self.in_progress.pop();
        self.current_program = previous_program;
        unwrapped
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
