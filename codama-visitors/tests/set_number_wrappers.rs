use codama_nodes::{
    AccountNode, AmountTypeNode, DateTimeTypeNode, NestedTypeNodeTrait, NumberTypeNode,
    ProgramNode, RootNode, SolAmountTypeNode, StructFieldTypeNode, StructTypeNode, TypeNode, U64,
};
use codama_visitors::{set_number_wrappers, NumberWrapper, TransformVisitor};
use pretty_assertions::assert_eq;

/// A program with two accounts, each a struct holding one u64 number field.
fn sample_root() -> RootNode {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.accounts.push(AccountNode::new(
        "token",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            NumberTypeNode::le(U64),
        )]),
    ));
    program.accounts.push(AccountNode::new(
        "other",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "balance",
            NumberTypeNode::le(U64),
        )]),
    ));
    RootNode::new(program)
}

fn account_field_type(root: &RootNode, account: usize, field: usize) -> TypeNode {
    let data: &StructTypeNode = root.program.accounts[account].data.get_nested_type_node();
    (*data.fields[field].r#type).clone()
}

#[test]
fn wraps_a_number_as_an_amount() {
    let mut visitor = set_number_wrappers([(
        "[accountNode]token",
        NumberWrapper::Amount {
            decimals: 9,
            unit: Some("USD".into()),
        },
    )]);
    let root = visitor.visit_root(sample_root());

    assert_eq!(
        account_field_type(&root, 0, 0),
        TypeNode::Amount(AmountTypeNode::new(
            NumberTypeNode::le(U64),
            9,
            Some("USD".into())
        )),
    );
}

#[test]
fn wraps_as_sol_amount_and_date_time() {
    let mut sol = set_number_wrappers([("[accountNode]token", NumberWrapper::SolAmount)]);
    let root = sol.visit_root(sample_root());
    assert_eq!(
        account_field_type(&root, 0, 0),
        TypeNode::SolAmount(SolAmountTypeNode::new(NumberTypeNode::le(U64))),
    );

    let mut dt = set_number_wrappers([("[accountNode]token", NumberWrapper::DateTime)]);
    let root = dt.visit_root(sample_root());
    assert_eq!(
        account_field_type(&root, 0, 0),
        TypeNode::DateTime(DateTimeTypeNode::new(NumberTypeNode::le(U64))),
    );
}

#[test]
fn selector_scopes_which_numbers_are_wrapped() {
    let mut visitor = set_number_wrappers([(
        "[accountNode]token",
        NumberWrapper::Amount {
            decimals: 2,
            unit: None,
        },
    )]);
    let root = visitor.visit_root(sample_root());

    // `token.amount` was wrapped...
    assert!(matches!(
        account_field_type(&root, 0, 0),
        TypeNode::Amount(_)
    ));
    // ...but `other.balance` (a different account) was left as a plain number.
    assert_eq!(
        account_field_type(&root, 1, 0),
        TypeNode::Number(NumberTypeNode::le(U64)),
    );
}
