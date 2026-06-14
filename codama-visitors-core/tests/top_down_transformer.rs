use codama_nodes::{AccountNode, ProgramNode, RootNode, StructTypeNode};
use codama_visitors_core::{
    bottom_up_transformer, top_down_transformer, TransformRule, TransformVisitor,
};
use pretty_assertions::assert_eq;
use std::cell::RefCell;
use std::rc::Rc;

fn sample_root() -> RootNode {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .accounts
        .push(AccountNode::new("acc", StructTypeNode::new(vec![])));
    RootNode::new(program)
}

/// Builds two rules (program + account) that record the order in which they run.
fn order_rules(order: &Rc<RefCell<Vec<&'static str>>>) -> Vec<TransformRule> {
    let program_order = Rc::clone(order);
    let account_order = Rc::clone(order);
    vec![
        TransformRule::new("[programNode]", move |node, _path| {
            program_order.borrow_mut().push("program");
            node
        }),
        TransformRule::new("[accountNode]", move |node, _path| {
            account_order.borrow_mut().push("account");
            node
        }),
    ]
}

#[test]
fn top_down_visits_parents_before_children() {
    let order = Rc::new(RefCell::new(Vec::new()));
    top_down_transformer(order_rules(&order)).visit_root(sample_root());
    assert_eq!(*order.borrow(), vec!["program", "account"]);
}

#[test]
fn bottom_up_visits_children_before_parents() {
    let order = Rc::new(RefCell::new(Vec::new()));
    bottom_up_transformer(order_rules(&order)).visit_root(sample_root());
    assert_eq!(*order.borrow(), vec!["account", "program"]);
}

#[test]
fn top_down_still_applies_transforms() {
    let mut transformer = top_down_transformer(vec![TransformRule::new(
        "[accountNode]acc",
        |node, _path| match node {
            codama_nodes::Node::Account(mut account) => {
                account.name = "renamed".into();
                codama_nodes::Node::Account(account)
            }
            other => other,
        },
    )]);
    let root = transformer.visit_root(sample_root());
    assert_eq!(root.program.accounts[0].name.as_ref(), "renamed");
}
