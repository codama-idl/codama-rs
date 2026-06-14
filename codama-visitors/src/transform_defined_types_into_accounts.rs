use codama_nodes::{AccountNode, CamelCaseString, DefinedTypeNode, Node, TypeNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};
use std::collections::HashSet;

/// Moves the named struct defined types out of `definedTypes` and into
/// `accounts` (with no discriminators or size).
///
/// The Rust counterpart of `@codama/visitors`' `transformDefinedTypesIntoAccountsVisitor`.
/// Operates per program. A named type whose body is not a struct is left as a
/// defined type (upstream asserts it is a struct).
///
/// ```
/// use codama_nodes::{DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U8};
/// use codama_visitors::{transform_defined_types_into_accounts, TransformVisitor};
///
/// let data = StructTypeNode::new(vec![StructFieldTypeNode::new("x", NumberTypeNode::le(U8))]);
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.defined_types.push(DefinedTypeNode::new("counter", data));
/// let root = RootNode::new(program);
///
/// let root = transform_defined_types_into_accounts(["counter"]).visit_root(root);
/// assert_eq!(root.program.accounts[0].name.as_ref(), "counter");
/// assert!(root.program.defined_types.is_empty());
/// ```
pub fn transform_defined_types_into_accounts<S: Into<String>>(
    names: impl IntoIterator<Item = S>,
) -> BottomUpTransformer {
    let names: HashSet<String> = names
        .into_iter()
        .map(|n| CamelCaseString::new(n.into()).to_string())
        .collect();

    bottom_up_transformer(vec![TransformRule::new(
        "[programNode]",
        move |node, _path| {
            let Node::Program(mut program) = node else {
                return node;
            };

            let mut kept: Vec<DefinedTypeNode> = Vec::new();
            let mut new_accounts: Vec<AccountNode> = Vec::new();
            for defined_type in program.defined_types.drain(..) {
                if names.contains(defined_type.name.as_ref()) {
                    match *defined_type.r#type {
                        TypeNode::Struct(data) => new_accounts.push(AccountNode {
                            name: defined_type.name,
                            docs: defined_type.docs,
                            size: None,
                            data: data.into(),
                            pda: None,
                            discriminators: Vec::new(),
                        }),
                        // Not a struct: cannot become an account, keep it as-is.
                        other => kept.push(DefinedTypeNode {
                            name: defined_type.name,
                            docs: defined_type.docs,
                            r#type: Box::new(other),
                        }),
                    }
                } else {
                    kept.push(defined_type);
                }
            }

            program.defined_types = kept;
            program.accounts.extend(new_accounts);
            Node::Program(program)
        },
    )])
}
