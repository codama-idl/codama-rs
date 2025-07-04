use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{
    ConstantPdaSeedNode, PdaNode, ProgramNode, RootNode, StringTypeNode, StringValueNode,
    VariablePdaSeedNode,
};

fn create_pda_with_single_seed(name: &str, seed_name: &str) -> PdaNode {
    PdaNode::new(
        name,
        vec![VariablePdaSeedNode::new(seed_name, StringTypeNode::utf8()).into()],
    )
}

#[test]
fn it_merges_pdas_into_root_nodes() {
    let member_pda = PdaNode::new(
        "member",
        vec![
            ConstantPdaSeedNode::new(StringTypeNode::utf8(), StringValueNode::new("member")).into(),
            VariablePdaSeedNode::new("identifier", StringTypeNode::utf8()).into(),
        ],
    );
    let person_pda = PdaNode::new(
        "person",
        vec![
            ConstantPdaSeedNode::new(StringTypeNode::utf8(), StringValueNode::new("person")).into(),
            VariablePdaSeedNode::new("firstname", StringTypeNode::utf8()).into(),
        ],
    );
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(member_pda.clone())
                .add_node(person_pda.clone())
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_pda(member_pda)
                    .add_pda(person_pda)
            )
            .into()
        )
    );
}

#[test]
fn it_merges_pdas_inside_programs_into_root_nodes() {
    let pda_a = create_pda_with_single_seed("foo", "seed");
    let pda_b = create_pda_with_single_seed("bar", "seed");
    let program_a = ProgramNode::default().add_pda(pda_a.clone());
    let program_b = ProgramNode::default().add_pda(pda_b.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_pda(pda_a).add_pda(pda_b)).into())
    );
}

#[test]
fn it_deduplicates_pdas_with_identical_names_by_using_the_last_one() {
    let first_counter = create_pda_with_single_seed("counter", "first");
    let second_counter = create_pda_with_single_seed("counter", "second");
    let third_counter = create_pda_with_single_seed("counter", "third");
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(first_counter)
                .add_node(second_counter)
                .add_node(third_counter.clone())
        ),
        Some(RootNode::new(ProgramNode::default().add_pda(third_counter)).into())
    );
}

#[test]
fn it_deduplicates_pdas_with_identical_names_inside_programs() {
    let first_counter = create_pda_with_single_seed("counter", "first");
    let second_counter = create_pda_with_single_seed("counter", "second");
    let program_a = ProgramNode::default().add_pda(first_counter);
    let program_b = ProgramNode::default().add_pda(second_counter.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_pda(second_counter)).into())
    );
}

#[test]
fn it_deduplicates_pdas_with_identical_names_with_an_initial_program_node() {
    let first_counter = create_pda_with_single_seed("counter", "first");
    let second_counter = create_pda_with_single_seed("counter", "second");
    let program_a = ProgramNode::default().add_pda(first_counter);
    let program_b = ProgramNode::default().add_pda(second_counter.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_pda(second_counter)).into())
    );
}
