use codama_nodes::{
    AccountNode, DiscriminatorNode, FieldDiscriminatorNode, NestedTypeNodeTrait, NumberTypeNode,
    NumberValueNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, ValueNode, U64,
    U8,
};
use codama_visitors::{
    set_account_discriminator_from_field, AccountDiscriminator, TransformVisitor,
};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let data = StructTypeNode::new(vec![
        StructFieldTypeNode::new("tag", NumberTypeNode::le(U8)),
        StructFieldTypeNode::new("amount", NumberTypeNode::le(U64)),
    ]);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.accounts.push(AccountNode::new("counter", data));
    RootNode::new(program)
}

#[test]
fn fixes_the_field_value_and_adds_a_field_discriminator() {
    let mut visitor = set_account_discriminator_from_field([(
        "counter",
        AccountDiscriminator::new("tag", NumberValueNode::new(7u8)),
    )]);
    let root = visitor.visit_root(sample_root());
    let account = &root.program.accounts[0];

    // A field discriminator on `tag` was prepended.
    assert_eq!(
        account.discriminators,
        vec![DiscriminatorNode::Field(FieldDiscriminatorNode::new(
            "tag", 0
        ))],
    );

    let fields = &account.data.get_nested_type_node().fields;
    // `tag` is now fixed to 7 (and omitted from the generated API)...
    assert_eq!(
        *fields[0].default_value,
        Some(ValueNode::Number(NumberValueNode::new(7u8))),
    );
    // ...while `amount` keeps no default value.
    assert_eq!(*fields[1].default_value, None);
}

#[test]
fn honours_a_custom_offset() {
    let mut visitor = set_account_discriminator_from_field([(
        "counter",
        AccountDiscriminator::new("tag", NumberValueNode::new(7u8)).offset(8),
    )]);
    let root = visitor.visit_root(sample_root());
    assert_eq!(
        root.program.accounts[0].discriminators,
        vec![DiscriminatorNode::Field(FieldDiscriminatorNode::new(
            "tag", 8
        ))],
    );
}

#[test]
fn leaves_the_account_unchanged_when_the_field_is_missing() {
    let mut visitor = set_account_discriminator_from_field([(
        "counter",
        AccountDiscriminator::new("nope", NumberValueNode::new(7u8)),
    )]);
    let root = visitor.visit_root(sample_root());
    assert!(root.program.accounts[0].discriminators.is_empty());
}
