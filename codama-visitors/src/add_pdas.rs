use codama_nodes::{Node, PdaNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};
use std::collections::HashSet;

/// Appends PDA definitions to programs selected by name.
///
/// The Rust counterpart of `@codama/visitors`' `addPdasVisitor`, built on
/// [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer). PDAs
/// whose name already exists on the program are skipped (upstream throws on a
/// name clash).
///
/// ```
/// use codama_nodes::{ConstantPdaSeedNode, NumberTypeNode, NumberValueNode, PdaNode, ProgramNode, RootNode, U8};
/// use codama_visitors::{add_pdas, TransformVisitor};
///
/// let pda = PdaNode::new("counter", vec![
///     ConstantPdaSeedNode::new(NumberTypeNode::le(U8), NumberValueNode::new(1u8)).into(),
/// ]);
/// let program = ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
/// let root = RootNode::new(program);
///
/// let root = add_pdas([("myProgram", vec![pda])]).visit_root(root);
/// assert_eq!(root.program.pdas.len(), 1);
/// ```
pub fn add_pdas<S: Into<String>>(
    map: impl IntoIterator<Item = (S, Vec<PdaNode>)>,
) -> BottomUpTransformer {
    let rules = map
        .into_iter()
        .map(|(program, pdas)| {
            TransformRule::new(
                format!("[programNode]{}", program.into()),
                move |node, _path| {
                    let Node::Program(mut program) = node else {
                        return node;
                    };
                    let existing: HashSet<_> =
                        program.pdas.iter().map(|pda| pda.name.clone()).collect();
                    for pda in pdas.clone() {
                        if !existing.contains(&pda.name) {
                            program.pdas.push(pda);
                        }
                    }
                    Node::Program(program)
                },
            )
        })
        .collect();
    bottom_up_transformer(rules)
}
