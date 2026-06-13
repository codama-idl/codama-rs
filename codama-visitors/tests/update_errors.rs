use codama_nodes::{ErrorNode, ProgramNode, RootNode};
use codama_visitors::{update_errors, ErrorUpdate, TransformVisitor};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.errors.push(ErrorNode {
        name: "invalidAuthority".into(),
        code: 1,
        message: "Invalid authority".into(),
        docs: Default::default(),
    });
    program.errors.push(ErrorNode {
        name: "alreadyClosed".into(),
        code: 2,
        message: "Already closed".into(),
        docs: Default::default(),
    });
    RootNode::new(program)
}

#[test]
fn updates_only_the_specified_fields_of_the_named_error() {
    let mut visitor = update_errors([(
        "invalidAuthority",
        ErrorUpdate::new()
            .code(42)
            .message("Authority does not match"),
    )]);
    let root = visitor.visit_root(sample_root());

    let error = &root.program.errors[0];
    assert_eq!(error.name.as_ref(), "invalidAuthority"); // unchanged
    assert_eq!(error.code, 42);
    assert_eq!(error.message, "Authority does not match");
}

#[test]
fn can_rename_an_error() {
    let mut visitor =
        update_errors([("invalidAuthority", ErrorUpdate::new().name("badAuthority"))]);
    let root = visitor.visit_root(sample_root());
    assert_eq!(root.program.errors[0].name.as_ref(), "badAuthority");
    assert_eq!(root.program.errors[0].code, 1); // other fields untouched
}

#[test]
fn leaves_unselected_errors_untouched() {
    let mut visitor = update_errors([("invalidAuthority", ErrorUpdate::new().code(99))]);
    let root = visitor.visit_root(sample_root());
    assert_eq!(root.program.errors[1].code, 2);
    assert_eq!(root.program.errors[1].name.as_ref(), "alreadyClosed");
}
