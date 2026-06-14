use codama_nodes::{PdaNode, ProgramNode, RootNode};
use codama_visitors::{add_pdas, TransformVisitor};
use pretty_assertions::assert_eq;

#[test]
fn appends_pdas_to_a_named_program_and_skips_duplicates() {
    let mut program = ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
    program.pdas.push(PdaNode::new("existing", vec![]));
    let root = RootNode::new(program);

    let root = add_pdas([(
        "myProgram",
        vec![
            PdaNode::new("existing", vec![]),
            PdaNode::new("fresh", vec![]),
        ],
    )])
    .visit_root(root);

    let names: Vec<_> = root
        .program
        .pdas
        .iter()
        .map(|p| p.name.to_string())
        .collect();
    assert_eq!(names, vec!["existing".to_string(), "fresh".to_string()]);
}

#[test]
fn does_not_touch_other_programs() {
    let program = ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
    let root = RootNode::new(program);
    let root = add_pdas([("other", vec![PdaNode::new("x", vec![])])]).visit_root(root);
    assert!(root.program.pdas.is_empty());
}
