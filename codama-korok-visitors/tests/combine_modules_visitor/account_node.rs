use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{
    AccountNode, BooleanTypeNode, NumberTypeNode, ProgramNode, RootNode, StringTypeNode,
    StructFieldTypeNode, StructTypeNode, U32,
};

fn create_account_with_single_field(name: &str, field_name: &str) -> AccountNode {
    AccountNode::new(
        name,
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            field_name,
            StringTypeNode::utf8(),
        )]),
    )
}

#[test]
fn it_merges_accounts_into_root_nodes() {
    let membership = AccountNode::new(
        "membership",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("identifier", StringTypeNode::utf8()),
            StructFieldTypeNode::new("active", BooleanTypeNode::default()),
        ]),
    );
    let person = AccountNode::new(
        "person",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
        ]),
    );
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(membership.clone())
                .add_node(person.clone())
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_account(membership)
                    .add_account(person)
            )
            .into()
        )
    );
}

#[test]
fn it_merges_accounts_inside_programs_into_root_nodes() {
    let empty_data = StructTypeNode::new(vec![]);
    let program_a = ProgramNode::default().add_account(AccountNode::new("foo", empty_data.clone()));
    let program_b = ProgramNode::default().add_account(AccountNode::new("bar", empty_data.clone()));
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_account(AccountNode::new("foo", empty_data.clone()))
                    .add_account(AccountNode::new("bar", empty_data))
            )
            .into()
        )
    );
}

#[test]
fn it_deduplicates_accounts_with_identical_names_by_using_the_last_one() {
    let first_counter = create_account_with_single_field("counter", "first");
    let second_counter = create_account_with_single_field("counter", "second");
    let third_counter = create_account_with_single_field("counter", "third");
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(first_counter)
                .add_node(second_counter)
                .add_node(third_counter.clone())
        ),
        Some(RootNode::new(ProgramNode::default().add_account(third_counter)).into())
    );
}

#[test]
fn it_deduplicates_accounts_with_identical_names_inside_programs() {
    let first_counter = create_account_with_single_field("counter", "first");
    let second_counter = create_account_with_single_field("counter", "second");
    let program_a = ProgramNode::default().add_account(first_counter);
    let program_b = ProgramNode::default().add_account(second_counter.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_account(second_counter)).into())
    );
}

#[test]
fn it_deduplicates_accounts_with_identical_names_with_an_initial_program_node() {
    let first_counter = create_account_with_single_field("counter", "first");
    let second_counter = create_account_with_single_field("counter", "second");
    let program_a = ProgramNode::default().add_account(first_counter);
    let program_b = ProgramNode::default().add_account(second_counter.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_account(second_counter)).into())
    );
}
