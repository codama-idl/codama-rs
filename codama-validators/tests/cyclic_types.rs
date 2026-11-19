//! The validation items of cyclic defined types, ported from the tests of `@codama/validators`.

use codama_nodes::{
    AccountLinkNode, AccountNode, ConstantValueNode, DefinedTypeLinkNode, DefinedTypeNode,
    HasTransforms, HiddenPrefixTransformNode, IntegerTypeNode, IntegerValueNode, NodeRef,
    OptionTypeNode, ProgramLinkNode, ProgramNode, PublicKeyTypeNode, RootNode, StructFieldTypeNode,
    StructTypeNode, U32, U8,
};
use codama_validators::{get_validation_items_visitor, LogLevel};
use codama_visitors_core::visit;

/// The level, message, path and cause of each item of `node`.
fn items<'a>(node: impl Into<NodeRef<'a>>) -> Vec<(LogLevel, String, String, Option<String>)> {
    visit(&get_validation_items_visitor(), node)
        .into_iter()
        .map(|item| {
            let cause = item.cause.as_ref().map(ToString::to_string);
            (item.level, item.message, item.path.to_string(), cause)
        })
        .collect()
}

/// The item of a defined type of `myProgram` that has no finite value.
fn no_finite_value(identifier: &str) -> (LogLevel, String, String, Option<String>) {
    let path = format!("[programNode]myProgram.[definedTypeNode]{identifier}");
    let message =
        format!("Defined type {path} has no finite value, so no codec can encode or decode it.");
    (LogLevel::Error, message.clone(), path, Some(message))
}

fn program(identifier: &'static str, defined_types: Vec<DefinedTypeNode>) -> ProgramNode {
    ProgramNode {
        defined_types,
        version: "1.0.0".into(),
        ..ProgramNode::new(identifier, "1111")
    }
}

/// A struct with a single field `next` of the given type.
fn next<T: Into<codama_nodes::TypeNode>>(r#type: T) -> StructTypeNode {
    StructTypeNode::new(vec![StructFieldTypeNode::new("next", r#type)])
}

#[test]
fn reports_cyclic_defined_types_without_a_finite_value() {
    // A loop whose every value nests another one.
    let root = RootNode::new(program(
        "myProgram",
        vec![DefinedTypeNode::new(
            "loop",
            next(DefinedTypeLinkNode::new("loop")),
        )],
    ));
    assert_eq!(items(&root), [no_finite_value("loop")]);
}

#[test]
fn reports_defined_types_aliasing_themselves() {
    let root = RootNode::new(program(
        "myProgram",
        vec![DefinedTypeNode::new(
            "alias",
            DefinedTypeLinkNode::new("alias"),
        )],
    ));
    assert_eq!(items(&root), [no_finite_value("alias")]);
}

#[test]
fn reports_defined_types_hiding_a_constant_of_themselves_in_a_link_transform() {
    // Encoding `self` requires a constant `self` value, which can never be created.
    let prefix = HiddenPrefixTransformNode::new(vec![ConstantValueNode::new(
        DefinedTypeLinkNode::new("self"),
        IntegerValueNode::new(0),
    )]);
    let root = RootNode::new(program(
        "myProgram",
        vec![
            DefinedTypeNode::new(
                "self",
                next(DefinedTypeLinkNode::new("other").add_transform(prefix)),
            ),
            DefinedTypeNode::new("other", IntegerTypeNode::le(U8)),
        ],
    ));
    assert_eq!(items(&root), [no_finite_value("self")]);
}

#[test]
fn accepts_cyclic_defined_types_with_a_finite_value() {
    // A linked list that may end with `None`.
    let root = RootNode::new(program(
        "myProgram",
        vec![DefinedTypeNode::new(
            "list",
            next(OptionTypeNode {
                prefix: IntegerTypeNode::le(U8),
                ..OptionTypeNode::new(DefinedTypeLinkNode::new("list"))
            }),
        )],
    ));
    assert_eq!(items(&root), []);
}

#[test]
fn accepts_accounts_whose_data_links_back_to_themselves() {
    // Nodes linking to their parent node through a defined type: public keys have a finite size.
    let parent = PublicKeyTypeNode {
        account_link: Some(AccountLinkNode::new("node")),
        ..PublicKeyTypeNode::new()
    };
    let root = RootNode::new(ProgramNode {
        accounts: vec![AccountNode::new(
            "node",
            DefinedTypeLinkNode::new("nodeData"),
        )],
        ..program(
            "myProgram",
            vec![DefinedTypeNode::new(
                "nodeData",
                StructTypeNode::new(vec![StructFieldTypeNode::new("parent", parent)]),
            )],
        )
    });
    assert_eq!(items(&root), []);
}

#[test]
fn only_reports_the_cyclic_defined_type_when_another_one_links_to_it() {
    let root = RootNode::new(program(
        "myProgram",
        vec![
            DefinedTypeNode::new("loop", next(DefinedTypeLinkNode::new("loop"))),
            DefinedTypeNode::new("usesLoop", DefinedTypeLinkNode::new("loop")),
        ],
    ));
    assert_eq!(items(&root), [no_finite_value("loop")]);
}

#[test]
fn reports_every_defined_type_of_a_cycle_without_a_finite_value() {
    // Two types nesting each other.
    let root = RootNode::new(program(
        "myProgram",
        vec![
            DefinedTypeNode::new("a", next(DefinedTypeLinkNode::new("b"))),
            DefinedTypeNode::new("b", next(DefinedTypeLinkNode::new("a"))),
        ],
    ));
    assert_eq!(items(&root), [no_finite_value("a"), no_finite_value("b")]);
}

#[test]
fn does_not_mistake_links_to_same_named_defined_types_of_other_programs_for_cycles() {
    let root = RootNode {
        additional_programs: vec![ProgramNode {
            public_key: "2222".into(),
            ..program(
                "programB",
                vec![DefinedTypeNode::new("config", IntegerTypeNode::le(U32))],
            )
        }],
        ..RootNode::new(program(
            "programA",
            vec![DefinedTypeNode::new(
                "config",
                DefinedTypeLinkNode::new_from_program("config", ProgramLinkNode::new("programB")),
            )],
        ))
    };
    assert_eq!(items(&root), []);
}

#[test]
fn does_not_check_the_values_of_defined_types_outside_a_program() {
    // A loop validated on its own only reports the link it cannot resolve without a program.
    let r#loop = DefinedTypeNode::new("loop", next(DefinedTypeLinkNode::new("loop")));
    assert_eq!(
        items(&r#loop),
        [(
            LogLevel::Error,
            "Pointing to a missing defined type named \"loop\"".to_string(),
            "[definedTypeNode]loop.[structTypeNode].[structFieldTypeNode]next.[definedTypeLinkNode]loop"
                .to_string(),
            None
        )]
    );
}
