use codama_nodes::{
    AccountNode, BytesTypeNode, DefinedTypeLinkNode, DefinedTypeNode, NumberTypeNode, ProgramNode,
    RootNode, StructFieldTypeNode, StructTypeNode, U32, U64, U8,
};
use codama_visitors::set_fixed_account_sizes;
use pretty_assertions::assert_eq;

fn field(name: &str, ty: impl Into<codama_nodes::TypeNode>) -> StructFieldTypeNode {
    StructFieldTypeNode::new(name, ty)
}

#[test]
fn sizes_fixed_accounts_and_skips_variable_ones() {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.accounts.push(AccountNode::new(
        "fixed",
        StructTypeNode::new(vec![
            field("a", NumberTypeNode::le(U32)),
            field("b", NumberTypeNode::le(U8)),
        ]),
    ));
    program.accounts.push(AccountNode::new(
        "variable",
        StructTypeNode::new(vec![field("blob", BytesTypeNode::new())]),
    ));

    let root = set_fixed_account_sizes(RootNode::new(program));
    assert_eq!(root.program.accounts[0].size, Some(5));
    assert_eq!(root.program.accounts[1].size, None);
}

#[test]
fn resolves_defined_type_links_when_sizing() {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("amount", NumberTypeNode::le(U64)));
    program.accounts.push(AccountNode::new(
        "wrapped",
        StructTypeNode::new(vec![field(
            "x",
            DefinedTypeLinkNode {
                name: "amount".into(),
                program: None,
            },
        )]),
    ));

    let root = set_fixed_account_sizes(RootNode::new(program));
    assert_eq!(root.program.accounts[0].size, Some(8));
}
