//! The tests of `getDefinedTypeLinksVisitor`, `isDefinedTypeCyclic`, `getHasFiniteValueVisitor` and
//! `hasDefinedTypeFiniteValue` in `@codama/visitors-core`.

use codama_nodes::{
    ArrayTypeNode, ConstantValueNode, DefinedTypeLinkNode, DefinedTypeNode, EnumTypeNode,
    EnumValueNode, EnumVariantTypeNode, FixedCountNode, HasTransforms, HiddenPrefixTransformNode,
    IntegerTypeNode, IntegerValueNode, MapTypeNode, NodeRef, OptionTypeNode, PrefixedCountNode,
    ProgramLinkNode, ProgramNode, RemainderCountNode, RemainderOptionTypeNode, RootNode,
    SetTypeNode, StringTypeNode, StringValueNode, StructFieldTypeNode, StructTypeNode,
    TupleTypeNode, TypeNode, ZeroableOptionTypeNode, U32, U64, U8,
};
use codama_visitors_core::{
    get_defined_type_links_visitor, get_has_finite_value_visitor, get_linkables_from_node,
    has_defined_type_finite_value, is_defined_type_cyclic, visit, LinkableDictionary, NodePath,
    NodeStack,
};

fn program(defined_types: Vec<DefinedTypeNode>) -> ProgramNode {
    ProgramNode {
        defined_types,
        ..ProgramNode::new("test", "1111")
    }
}

/// The path of the defined type named `identifier`, from the root through its program.
fn type_path<'a>(root: &'a RootNode, identifier: &str) -> NodePath<'a> {
    std::iter::once(&root.program)
        .chain(&root.additional_programs)
        .find_map(|program| {
            let defined_type = program
                .defined_types
                .iter()
                .find(|defined_type| defined_type.identifier.as_ref() == identifier)?;
            Some(NodePath::from(vec![
                NodeRef::from(root),
                NodeRef::from(program),
                NodeRef::from(defined_type),
            ]))
        })
        .expect("the defined type exists")
}

fn field<T: Into<TypeNode>>(r#type: T) -> StructTypeNode {
    StructTypeNode::new(vec![StructFieldTypeNode::new("value", r#type)])
}

fn prefixed_option<T: Into<TypeNode>>(item: T) -> OptionTypeNode {
    OptionTypeNode {
        prefix: IntegerTypeNode::le(U8),
        ..OptionTypeNode::new(item)
    }
}

fn links(root: &RootNode, identifier: &str) -> Vec<String> {
    let linkables = get_linkables_from_node(root);
    let stack = NodeStack::default();
    let visitor = get_defined_type_links_visitor(&linkables, &stack);
    stack
        .visit_path(type_path(root, identifier), |node| visit(&visitor, node))
        .iter()
        .map(ToString::to_string)
        .collect()
}

fn is_cyclic(root: &RootNode, identifier: &str) -> bool {
    is_defined_type_cyclic(&type_path(root, identifier), &get_linkables_from_node(root)).unwrap()
}

fn has_finite_value(root: &RootNode, identifier: &str) -> bool {
    let linkables = get_linkables_from_node(root);
    let stack = NodeStack::default();
    let visitor = get_has_finite_value_visitor(&linkables, &stack);
    stack.visit_path(type_path(root, identifier), |node| visit(&visitor, node))
}

// getDefinedTypeLinksVisitor.

#[test]
fn lists_the_defined_types_linked_by_a_type_without_following_them() {
    // A type linking to an alias, to a type of another program within a transform, and to itself.
    let label = StringTypeNode::utf8().add_transform(HiddenPrefixTransformNode::new(vec![
        ConstantValueNode::new(
            DefinedTypeLinkNode::new_from_program("length", ProgramLinkNode::new("other")),
            IntegerValueNode::new(0),
        ),
    ]));
    let node = DefinedTypeNode::new(
        "node",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("value", DefinedTypeLinkNode::new("amount")),
            StructFieldTypeNode::new("label", label),
            StructFieldTypeNode::new("next", prefixed_option(DefinedTypeLinkNode::new("node"))),
        ]),
    );
    let root = RootNode {
        additional_programs: vec![ProgramNode {
            defined_types: vec![DefinedTypeNode::new("length", IntegerTypeNode::le(U32))],
            ..ProgramNode::new("other", "2222")
        }],
        ..RootNode::new(program(vec![
            node,
            DefinedTypeNode::new("amount", IntegerTypeNode::le(U64)),
        ]))
    };
    assert_eq!(
        links(&root, "node"),
        [
            "[programNode]test.[definedTypeNode]amount",
            "[programNode]other.[definedTypeNode]length",
            "[programNode]test.[definedTypeNode]node",
        ]
    );
}

#[test]
fn ignores_the_defined_types_of_enum_values_and_unresolved_links() {
    // An enum whose variant defaults to one of its own values, next to a missing link.
    let down = StructTypeNode::new(vec![
        StructFieldTypeNode {
            default_value: Box::new(Some(
                EnumValueNode::new(DefinedTypeLinkNode::new("direction"), "up").into(),
            )),
            ..StructFieldTypeNode::new("then", IntegerTypeNode::le(U8))
        },
        StructFieldTypeNode::new("missing", DefinedTypeLinkNode::new("missing")),
    ]);
    let root = RootNode::new(program(vec![DefinedTypeNode::new(
        "direction",
        EnumTypeNode::new(vec![
            EnumVariantTypeNode::new("up"),
            EnumVariantTypeNode::with_data("down", down),
        ]),
    )]));
    assert!(links(&root, "direction").is_empty());
}

#[test]
fn lists_links_nested_deep_within_a_type() {
    // A link nested within an option, a tuple, an enum variant and a struct.
    let paid = TupleTypeNode::new(vec![
        IntegerTypeNode::le(U8).into(),
        prefixed_option(DefinedTypeLinkNode::new("amount")).into(),
    ]);
    let payment = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "kind",
        EnumTypeNode::new(vec![
            EnumVariantTypeNode::new("free"),
            EnumVariantTypeNode::with_data("paid", paid),
        ]),
    )]);
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new("payment", payment),
        DefinedTypeNode::new("amount", IntegerTypeNode::le(U64)),
    ]));
    assert_eq!(
        links(&root, "payment"),
        ["[programNode]test.[definedTypeNode]amount"]
    );
}

#[test]
fn cannot_resolve_links_when_visited_without_the_path_of_the_type() {
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new("wrapper", DefinedTypeLinkNode::new("amount")),
        DefinedTypeNode::new("amount", IntegerTypeNode::le(U64)),
    ]));
    let linkables = get_linkables_from_node(&root);
    let stack = NodeStack::default();
    let visitor = get_defined_type_links_visitor(&linkables, &stack);

    // Visited on its own, without its program on the path, its link cannot be resolved.
    assert!(visit(&visitor, &root.program.defined_types[0]).is_empty());
}

// isDefinedTypeCyclic.

#[test]
fn detects_defined_types_linking_to_themselves() {
    let root = RootNode::new(program(vec![DefinedTypeNode::new(
        "list",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("value", IntegerTypeNode::le(U8)),
            StructFieldTypeNode::new("next", prefixed_option(DefinedTypeLinkNode::new("list"))),
        ]),
    )]));
    assert!(is_cyclic(&root, "list"));
}

#[test]
fn detects_cycles_through_other_defined_types() {
    // A tree linking to a forest that links back to the tree.
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new(
            "tree",
            StructTypeNode::new(vec![StructFieldTypeNode::new(
                "children",
                DefinedTypeLinkNode::new("forest"),
            )]),
        ),
        DefinedTypeNode::new(
            "forest",
            ArrayTypeNode::new(
                DefinedTypeLinkNode::new("tree"),
                PrefixedCountNode::new(IntegerTypeNode::le(U32)),
            ),
        ),
    ]));
    assert!(is_cyclic(&root, "tree"));
}

#[test]
fn does_not_mark_types_linking_to_a_cycle_as_cyclic() {
    // A leaf linking to a list, which links to itself.
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new("list", prefixed_option(DefinedTypeLinkNode::new("list"))),
        DefinedTypeNode::new(
            "leaf",
            StructTypeNode::new(vec![StructFieldTypeNode::new(
                "list",
                DefinedTypeLinkNode::new("list"),
            )]),
        ),
    ]));
    assert!(!is_cyclic(&root, "leaf"));
}

#[test]
fn detects_cycles_through_transforms() {
    // A string hiding a constant prefix of its own type.
    let tagged = StringTypeNode::utf8().add_transform(HiddenPrefixTransformNode::new(vec![
        ConstantValueNode::new(
            DefinedTypeLinkNode::new("tagged"),
            StringValueNode::new("x"),
        ),
    ]));
    let root = RootNode::new(program(vec![DefinedTypeNode::new("tagged", tagged)]));
    assert!(is_cyclic(&root, "tagged"));
}

#[test]
fn ignores_links_within_values() {
    // An enum whose struct variant defaults to one of its own values.
    let down = StructTypeNode::new(vec![StructFieldTypeNode {
        default_value: Box::new(Some(
            EnumValueNode::new(DefinedTypeLinkNode::new("direction"), "up").into(),
        )),
        ..StructFieldTypeNode::new("then", IntegerTypeNode::le(U8))
    }]);
    let root = RootNode::new(program(vec![DefinedTypeNode::new(
        "direction",
        EnumTypeNode::new(vec![
            EnumVariantTypeNode::new("up"),
            EnumVariantTypeNode::with_data("down", down),
        ]),
    )]));

    // Values do not shape the type.
    assert!(!is_cyclic(&root, "direction"));
}

#[test]
fn does_not_confuse_same_named_defined_types_of_different_programs() {
    // A type of program A linking to a same-named type of program B.
    let root = RootNode {
        additional_programs: vec![ProgramNode {
            defined_types: vec![DefinedTypeNode::new("config", IntegerTypeNode::le(U32))],
            ..ProgramNode::new("b", "2222")
        }],
        ..RootNode::new(ProgramNode {
            defined_types: vec![DefinedTypeNode::new(
                "config",
                DefinedTypeLinkNode::new_from_program("config", ProgramLinkNode::new("b")),
            )],
            ..ProgramNode::new("a", "1111")
        })
    };
    assert!(!is_cyclic(&root, "config"));
}

#[test]
fn ignores_links_it_cannot_resolve() {
    let root = RootNode::new(program(vec![DefinedTypeNode::new(
        "orphan",
        DefinedTypeLinkNode::new("missing"),
    )]));
    assert!(!is_cyclic(&root, "orphan"));
}

#[test]
fn sees_types_recorded_after_a_previous_call() {
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new("a", DefinedTypeLinkNode::new("b")),
        DefinedTypeNode::new("b", DefinedTypeLinkNode::new("a")),
    ]));
    let linkables = LinkableDictionary::default();
    linkables.record_path(&type_path(&root, "a"));
    assert!(!is_defined_type_cyclic(&type_path(&root, "a"), &linkables).unwrap());

    // Once the missing type is recorded, the cycle is found.
    linkables.record_path(&type_path(&root, "b"));
    assert!(is_defined_type_cyclic(&type_path(&root, "a"), &linkables).unwrap());
}

#[test]
fn fails_when_the_path_has_no_program() {
    let root = RootNode::new(program(vec![DefinedTypeNode::new(
        "list",
        prefixed_option(DefinedTypeLinkNode::new("list")),
    )]));
    let linkables = get_linkables_from_node(&root);

    // Checked from a path without its program, it fails rather than missing the cycle.
    let path = NodePath::from(vec![NodeRef::from(&root.program.defined_types[0])]);
    let expected = "Expected a node path containing a program node, e.g. to resolve the links of its last node, got [definedTypeNode]list.";
    assert_eq!(
        is_defined_type_cyclic(&path, &linkables)
            .unwrap_err()
            .to_string(),
        expected
    );
    assert_eq!(
        has_defined_type_finite_value(&path, &linkables)
            .unwrap_err()
            .to_string(),
        expected
    );
}

#[test]
fn fails_when_the_path_does_not_end_with_a_defined_type() {
    let root = RootNode::new(program(vec![]));
    let linkables = get_linkables_from_node(&root);
    let path = NodePath::from(vec![NodeRef::from(&root), NodeRef::from(&root.program)]);
    let expected = "Expected node of kind [definedTypeNode], got [programNode]test.";
    assert_eq!(
        is_defined_type_cyclic(&path, &linkables)
            .unwrap_err()
            .to_string(),
        expected
    );
    assert_eq!(
        has_defined_type_finite_value(&path, &linkables)
            .unwrap_err()
            .to_string(),
        expected
    );
}

// getHasFiniteValueVisitor.

/// A defined type named `self` with the given type, so `link(self)` links back to it.
fn self_root<T: Into<TypeNode>>(r#type: T) -> RootNode {
    RootNode::new(program(vec![DefinedTypeNode::new("self", r#type)]))
}

fn link_to_self() -> DefinedTypeLinkNode {
    DefinedTypeLinkNode::new("self")
}

#[test]
fn finds_a_finite_value_through_types_that_may_stop_a_recursion() {
    let u8 = || IntegerTypeNode::le(U8);
    let cases: Vec<(&str, TypeNode)> = vec![
        ("non-cyclic types", field(u8()).into()),
        ("options", field(prefixed_option(link_to_self())).into()),
        (
            "remainder options",
            field(RemainderOptionTypeNode::new(link_to_self())).into(),
        ),
        (
            "zeroable options",
            field(ZeroableOptionTypeNode::new(link_to_self())).into(),
        ),
        (
            "prefixed arrays",
            ArrayTypeNode::new(link_to_self(), PrefixedCountNode::new(u8())).into(),
        ),
        (
            "remainder sets",
            SetTypeNode::new(link_to_self(), RemainderCountNode::new()).into(),
        ),
        (
            "empty fixed arrays",
            ArrayTypeNode::new(link_to_self(), FixedCountNode::new(0)).into(),
        ),
        (
            "prefixed maps",
            MapTypeNode::new(u8(), link_to_self(), PrefixedCountNode::new(u8())).into(),
        ),
        (
            "enums with another variant",
            EnumTypeNode::new(vec![
                EnumVariantTypeNode::new("nil"),
                EnumVariantTypeNode::with_data("cons", field(link_to_self())),
            ])
            .into(),
        ),
        (
            "links it cannot resolve",
            field(DefinedTypeLinkNode::new("missing")).into(),
        ),
    ];
    for (case, r#type) in cases {
        assert!(has_finite_value(&self_root(r#type), "self"), "{case}");
    }
}

#[test]
fn finds_no_finite_value_through_types_that_never_stop_a_recursion() {
    let u8 = || IntegerTypeNode::le(U8);
    let cases: Vec<(&str, TypeNode)> = vec![
        ("aliases of themselves", link_to_self().into()),
        ("struct fields", field(link_to_self()).into()),
        (
            "tuple items",
            TupleTypeNode::new(vec![u8().into(), link_to_self().into()]).into(),
        ),
        (
            "fixed arrays",
            ArrayTypeNode::new(link_to_self(), FixedCountNode::new(2)).into(),
        ),
        (
            "fixed maps",
            MapTypeNode::new(link_to_self(), u8(), FixedCountNode::new(1)).into(),
        ),
        (
            "enums whose every variant recurses",
            EnumTypeNode::new(vec![EnumVariantTypeNode::with_data(
                "cons",
                field(link_to_self()),
            )])
            .into(),
        ),
        (
            "enums without variants",
            field(EnumTypeNode::new(vec![])).into(),
        ),
    ];
    for (case, r#type) in cases {
        assert!(!has_finite_value(&self_root(r#type), "self"), "{case}");
    }
}

#[test]
fn finds_no_finite_value_through_a_type_linking_to_one_without() {
    // A type that is not cyclic itself, but links to a loop.
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new("loop", field(DefinedTypeLinkNode::new("loop"))),
        DefinedTypeNode::new("usesLoop", DefinedTypeLinkNode::new("loop")),
    ]));
    assert!(!has_finite_value(&root, "usesLoop"));
}

#[test]
fn finds_no_finite_value_through_a_cycle_across_defined_types_without_a_way_out() {
    // `a -> b -> a` through struct fields only.
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new("a", field(DefinedTypeLinkNode::new("b"))),
        DefinedTypeNode::new("b", field(DefinedTypeLinkNode::new("a"))),
    ]));
    assert!(!has_finite_value(&root, "a"));
}

/// A tree whose recursive variant goes through a branch, and whose other variant ends.
fn tree_and_branch() -> Vec<DefinedTypeNode> {
    vec![
        DefinedTypeNode::new(
            "tree",
            EnumTypeNode::new(vec![
                EnumVariantTypeNode::with_data("node", field(DefinedTypeLinkNode::new("branch"))),
                EnumVariantTypeNode::new("leaf"),
            ]),
        ),
        DefinedTypeNode::new("branch", field(DefinedTypeLinkNode::new("tree"))),
    ]
}

#[test]
fn finds_a_finite_value_through_another_branch_of_a_cycle_across_defined_types() {
    // The branch only reaches the way out through the tree.
    let root = RootNode::new(program(tree_and_branch()));
    assert!(has_finite_value(&root, "branch"));
}

#[test]
fn finds_a_finite_value_through_types_shared_by_sibling_branches() {
    // A list whose two fields link to the same list type, each of them able to end.
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new(
            "list",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("left", prefixed_option(DefinedTypeLinkNode::new("list"))),
                StructFieldTypeNode::new("right", DefinedTypeLinkNode::new("leaf")),
            ]),
        ),
        DefinedTypeNode::new(
            "leaf",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("a", DefinedTypeLinkNode::new("shared")),
                StructFieldTypeNode::new("b", DefinedTypeLinkNode::new("shared")),
            ]),
        ),
        DefinedTypeNode::new("shared", prefixed_option(DefinedTypeLinkNode::new("list"))),
    ]));
    assert!(has_finite_value(&root, "list"));
}

#[test]
fn finds_no_finite_value_through_types_shared_by_sibling_branches_without_a_way_out() {
    // A loop whose two fields link to the same type, which links back to the loop.
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new(
            "loop",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("a", DefinedTypeLinkNode::new("shared")),
                StructFieldTypeNode::new("b", DefinedTypeLinkNode::new("shared")),
            ]),
        ),
        DefinedTypeNode::new("shared", field(DefinedTypeLinkNode::new("loop"))),
    ]));
    assert!(!has_finite_value(&root, "loop"));
}

#[test]
fn finds_a_finite_value_for_a_type_first_reached_within_a_cycle_it_does_not_need() {
    // A forest using both the tree and the branch, which it reaches first through the tree.
    let mut defined_types = tree_and_branch();
    defined_types.push(DefinedTypeNode::new(
        "forest",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("tree", DefinedTypeLinkNode::new("tree")),
            StructFieldTypeNode::new("branch", DefinedTypeLinkNode::new("branch")),
        ]),
    ));
    let root = RootNode::new(program(defined_types));

    // The branch is not mistaken for having no finite value.
    assert!(has_finite_value(&root, "forest"));
}

// hasDefinedTypeFiniteValue.

#[test]
fn has_defined_type_finite_value_follows_the_recorded_links() {
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new(
            "list",
            field(prefixed_option(DefinedTypeLinkNode::new("list"))),
        ),
        DefinedTypeNode::new("loop", field(DefinedTypeLinkNode::new("loop"))),
    ]));
    let linkables = get_linkables_from_node(&root);
    assert!(has_defined_type_finite_value(&type_path(&root, "list"), &linkables).unwrap());
    assert!(!has_defined_type_finite_value(&type_path(&root, "loop"), &linkables).unwrap());
}

#[test]
fn has_defined_type_finite_value_sees_types_recorded_after_a_previous_call() {
    let root = RootNode::new(program(vec![
        DefinedTypeNode::new("usesLoop", DefinedTypeLinkNode::new("loop")),
        DefinedTypeNode::new("loop", DefinedTypeLinkNode::new("loop")),
    ]));
    let linkables = LinkableDictionary::default();
    linkables.record_path(&type_path(&root, "usesLoop"));
    assert!(has_defined_type_finite_value(&type_path(&root, "usesLoop"), &linkables).unwrap());

    // Once the loop is recorded, it no longer has a finite value.
    linkables.record_path(&type_path(&root, "loop"));
    assert!(!has_defined_type_finite_value(&type_path(&root, "usesLoop"), &linkables).unwrap());
}
