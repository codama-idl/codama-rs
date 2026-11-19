//! The validation items of instruction inputs and injected values, ported from the tests of
//! `@codama/validators`.

use codama_errors::CodamaError;
use codama_nodes::{
    AccountBumpValueNode, AccountValueNode, InjectedValueNode, InstructionAccountNode,
    InstructionNode, IntegerTypeNode, IntegerValueNode, IsSigner, ProvidedNode,
    StructFieldTypeNode, StructTypeNode, ValueNode, U8,
};
use codama_validators::{get_validation_items_visitor, LogLevel};
use codama_visitors_core::visit;

fn instruction(identifier: &'static str) -> InstructionNode {
    InstructionNode {
        identifier: identifier.into(),
        ..InstructionNode::default()
    }
}

fn u8_field(identifier: &'static str, default_value: impl Into<ValueNode>) -> StructFieldTypeNode {
    StructFieldTypeNode {
        default_value: Box::new(Some(default_value.into())),
        ..StructFieldTypeNode::new(identifier, IntegerTypeNode::le(U8))
    }
}

#[test]
fn reports_errors_raised_when_resolving_instruction_default_values() {
    let node = InstructionNode {
        accounts: vec![
            InstructionAccountNode {
                default_value: Box::new(Some(AccountValueNode::new("b").into())),
                ..InstructionAccountNode::new("a", false, false)
            },
            InstructionAccountNode {
                default_value: Box::new(Some(AccountValueNode::new("a").into())),
                ..InstructionAccountNode::new("b", false, false)
            },
        ],
        ..instruction("myInstruction")
    };
    let items = visit(&get_validation_items_visitor(), &node);
    assert_eq!(items.len(), 1);
    let message = "Circular dependency detected when resolving the default values of instruction [instructionNode]myInstruction. Got the following dependency cycle [a -> b -> a].";
    assert_eq!(items[0].level, LogLevel::Error);
    assert_eq!(items[0].message, message);
    assert_eq!(items[0].path.to_string(), "[instructionNode]myInstruction");
    assert!(matches!(
        items[0].cause,
        Some(CodamaError::CyclicDependencyDetectedWhenResolvingInstructionDefaultValues { .. })
    ));
}

#[test]
fn reports_a_data_field_defaulting_to_the_bump_of_a_potential_signer() {
    let node = InstructionNode {
        accounts: vec![InstructionAccountNode::new(
            "authority",
            false,
            IsSigner::Either,
        )],
        data: Box::new(Some(
            StructTypeNode::new(vec![u8_field(
                "bump",
                InjectedValueNode::new("authorityBump"),
            )])
            .into(),
        )),
        provides: vec![ProvidedNode::new(
            "authorityBump",
            AccountBumpValueNode::new("authority"),
        )],
        ..instruction("myInstruction")
    };
    let items: Vec<_> = visit(&get_validation_items_visitor(), &node)
        .into_iter()
        .map(|item| item.to_string())
        .collect();
    assert_eq!(
        items,
        ["[error] Data field \"bump\" cannot default to the bump of the \"authority\" account as it may be a signer. at [instructionNode]myInstruction"]
    );
}

#[test]
fn reports_injected_values_that_are_neither_provided_nor_given_a_fallback() {
    // An unprovided injection, an injection with a fallback, and a sub-instruction consuming a key
    // that its parent provides.
    let node = InstructionNode {
        data: Box::new(Some(
            StructTypeNode::new(vec![
                u8_field("a", InjectedValueNode::new("missing")),
                u8_field(
                    "b",
                    InjectedValueNode::with_fallback("alsoMissing", IntegerValueNode::new(1)),
                ),
            ])
            .into(),
        )),
        provides: vec![ProvidedNode::new("decimals", IntegerValueNode::new(6))],
        sub_instructions: vec![InstructionNode {
            data: Box::new(Some(
                StructTypeNode::new(vec![u8_field(
                    "decimals",
                    InjectedValueNode::new("decimals"),
                )])
                .into(),
            )),
            ..instruction("child")
        }],
        ..instruction("parent")
    };
    let items: Vec<_> = visit(&get_validation_items_visitor(), &node)
        .into_iter()
        .map(|item| item.to_string())
        .collect();
    assert_eq!(
        items,
        ["[error] Injected value \"missing\" is not provided and has no fallback. at [instructionNode]parent.[structTypeNode].[structFieldTypeNode]a.[injectedValueNode]"]
    );
}

#[test]
fn does_not_report_injected_values_outside_of_instructions() {
    let field = u8_field("a", InjectedValueNode::new("missing"));
    assert!(visit(&get_validation_items_visitor(), &field).is_empty());
}
