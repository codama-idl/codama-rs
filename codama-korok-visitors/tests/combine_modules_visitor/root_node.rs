use super::utils::{combine_modules, get_type, CombineModulesInput};
use codama_nodes::{NumberFormat::U32, NumberTypeNode, ProgramNode, RootNode, StringValueNode};

#[test]
fn root_with_no_nodes_to_combines() {
    assert_eq!(combine_modules(CombineModulesInput::new()), None);
}

#[test]
fn root_with_no_relevant_nodes_to_combines() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(NumberTypeNode::le(U32))
                .add_node(StringValueNode::new("hello"))
        ),
        None
    );
}

#[test]
fn default_roots() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(RootNode::new(
                    ProgramNode::default().add_defined_type(get_type("type_1"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::default().add_defined_type(get_type("type_2"))
                ))
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_defined_type(get_type("type_1"))
                    .add_defined_type(get_type("type_2"))
            )
            .into()
        )
    );
}

#[test]
fn roots_with_same_pubkey_programs() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(RootNode::new(
                    ProgramNode::new("program_a", "1234").add_defined_type(get_type("type_1"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::new("program_b", "1234").add_defined_type(get_type("type_2"))
                ))
        ),
        Some(
            RootNode::new(
                ProgramNode::new("program_a", "1234")
                    .add_defined_type(get_type("type_1"))
                    .add_defined_type(get_type("type_2"))
            )
            .into()
        )
    );
}

#[test]
fn roots_with_different_pubkey_programs() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(RootNode::new(
                    ProgramNode::new("program_a", "1111").add_defined_type(get_type("type_1"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::new("program_b", "2222").add_defined_type(get_type("type_2"))
                ))
        ),
        Some(
            RootNode::new(
                ProgramNode::new("program_a", "1111").add_defined_type(get_type("type_1"))
            )
            .add_program(ProgramNode::new("program_b", "2222").add_defined_type(get_type("type_2")))
            .into()
        )
    );
}

#[test]
fn roots_with_additional_programs() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(
                    RootNode::new(
                        ProgramNode::new("program_a", "1111")
                            .add_defined_type(get_type("type_a_1"))
                    )
                    .add_program(
                        ProgramNode::new("program_b", "2222")
                            .add_defined_type(get_type("type_b_1"))
                    )
                    .add_program(
                        ProgramNode::new("program_c", "3333")
                            .add_defined_type(get_type("type_c_1"))
                    )
                )
                .add_node(
                    RootNode::new(
                        ProgramNode::new("program_b", "2222")
                            .add_defined_type(get_type("type_b_2"))
                    )
                    .add_program(
                        ProgramNode::new("program_d", "4444")
                            .add_defined_type(get_type("type_d_2"))
                    )
                    .add_program(
                        ProgramNode::new("program_a", "1111")
                            .add_defined_type(get_type("type_a_2"))
                    )
                )
        ),
        Some(
            RootNode::new(
                ProgramNode::new("program_a", "1111")
                    .add_defined_type(get_type("type_a_1"))
                    .add_defined_type(get_type("type_a_2"))
            )
            .add_program(
                ProgramNode::new("program_b", "2222")
                    .add_defined_type(get_type("type_b_1"))
                    .add_defined_type(get_type("type_b_2"))
            )
            .add_program(
                ProgramNode::new("program_c", "3333").add_defined_type(get_type("type_c_1"))
            )
            .add_program(
                ProgramNode::new("program_d", "4444").add_defined_type(get_type("type_d_2"))
            )
            .into()
        )
    );
}

#[test]
fn defined_root_within_scraps() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(get_type("scraps_1"))
                .add_node(RootNode::new(
                    ProgramNode::new("my_program", "1234").add_defined_type(get_type("type_1"))
                ))
                .add_node(get_type("scraps_2"))
        ),
        Some(
            RootNode::new(
                ProgramNode::new("my_program", "1234").add_defined_type(get_type("type_1"))
            )
            .add_program(
                ProgramNode::default()
                    .add_defined_type(get_type("scraps_1"))
                    .add_defined_type(get_type("scraps_2"))
            )
            .into()
        )
    );
}

#[test]
fn default_root_within_scraps() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(get_type("scraps_1"))
                .add_node(RootNode::default())
                .add_node(get_type("scraps_2"))
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_defined_type(get_type("scraps_1"))
                    .add_defined_type(get_type("scraps_2"))
            )
            .into()
        )
    );
}

#[test]
fn exisiting_irrelevant_node() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(StringValueNode::new("hello"))
                .add_node(ProgramNode::new("foo", "1111"))
                .add_node(ProgramNode::new("bar", "2222"))
        ),
        Some(StringValueNode::new("hello").into())
    );
}

#[test]
fn exisiting_default_roots() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(RootNode::new(
                    ProgramNode::default().add_defined_type(get_type("type_1"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::default().add_defined_type(get_type("type_2"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::default().add_defined_type(get_type("type_3"))
                ))
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_defined_type(get_type("type_1"))
                    .add_defined_type(get_type("type_2"))
                    .add_defined_type(get_type("type_3"))
            )
            .into()
        )
    );
}

#[test]
fn exisiting_roots_with_same_pubkeys() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(RootNode::new(
                    ProgramNode::new("program_a", "1234").add_defined_type(get_type("type_1"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::new("program_b", "1234").add_defined_type(get_type("type_2"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::new("program_c", "1234").add_defined_type(get_type("type_3"))
                ))
        ),
        Some(
            RootNode::new(
                ProgramNode::new("program_a", "1234")
                    .add_defined_type(get_type("type_1"))
                    .add_defined_type(get_type("type_2"))
                    .add_defined_type(get_type("type_3"))
            )
            .into()
        )
    );
}

#[test]
fn exisiting_roots_with_different_pubkeys() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(RootNode::new(
                    ProgramNode::new("program_a", "1111").add_defined_type(get_type("type_1"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::new("program_b", "2222").add_defined_type(get_type("type_2"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::new("program_c", "3333").add_defined_type(get_type("type_3"))
                ))
        ),
        Some(
            RootNode::new(
                ProgramNode::new("program_a", "1111").add_defined_type(get_type("type_1"))
            )
            .add_program(ProgramNode::new("program_b", "2222").add_defined_type(get_type("type_2")))
            .add_program(ProgramNode::new("program_c", "3333").add_defined_type(get_type("type_3")))
            .into()
        )
    );
}

#[test]
fn exisiting_root_with_pubkey_less_child_root() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(RootNode::new(
                    ProgramNode::new("my_program", "1234").add_defined_type(get_type("type_1"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::default().add_defined_type(get_type("type_2"))
                ))
                .add_node(RootNode::new(
                    ProgramNode::default().add_defined_type(get_type("type_3"))
                ))
        ),
        Some(
            RootNode::new(
                ProgramNode::new("my_program", "1234")
                    .add_defined_type(get_type("type_1"))
                    .add_defined_type(get_type("type_2"))
                    .add_defined_type(get_type("type_3"))
            )
            .into()
        )
    );
}

#[test]
fn existing_defined_root_with_scraps() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(RootNode::new(
                    ProgramNode::new("program_a", "1111").add_defined_type(get_type("type_1"))
                ))
                .add_node(get_type("scraps_1"))
                .add_node(RootNode::new(
                    ProgramNode::new("program_b", "2222").add_defined_type(get_type("type_2"))
                ))
                .add_node(get_type("scraps_2"))
        ),
        Some(
            RootNode::new(
                ProgramNode::new("program_a", "1111")
                    .add_defined_type(get_type("type_1"))
                    .add_defined_type(get_type("scraps_1"))
                    .add_defined_type(get_type("scraps_2"))
            )
            .add_program(ProgramNode::new("program_b", "2222").add_defined_type(get_type("type_2")))
            .into()
        )
    );
}

#[test]
fn existing_default_root_with_scraps() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(RootNode::default())
                .add_node(get_type("scraps_1"))
                .add_node(RootNode::new(
                    ProgramNode::new("program_b", "2222").add_defined_type(get_type("type_2"))
                ))
                .add_node(get_type("scraps_2"))
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_defined_type(get_type("scraps_1"))
                    .add_defined_type(get_type("scraps_2"))
            )
            .add_program(ProgramNode::new("program_b", "2222").add_defined_type(get_type("type_2")))
            .into()
        )
    );
}
