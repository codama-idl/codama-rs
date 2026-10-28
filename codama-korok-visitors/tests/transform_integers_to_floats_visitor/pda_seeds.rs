use super::utils::{
    constant_with_defined_type_link, float, instruction_with_account_default, integer,
    pda_link_with_program, root_with_programs, transform_integers_to_floats,
};
use crate::helpers::path;
use codama_nodes::{
    AccountValueNode, ConstantPdaSeedNode, DataValueNode, DefinedTypeLinkNode, DefinedTypeNode,
    FloatFormat::{F32, F64},
    FloatTypeNode, FloatValueNode, HasTransforms, HiddenPrefixTransformNode, InstructionNode,
    IntegerFormat::U64,
    IntegerTypeNode, PdaLinkNode, PdaNode, PdaSeedValueNode, PdaSeedValueValue, PdaValueNode,
    PdaValuePda, ProgramNode, ProvidedNode, StringTypeNode, StringValueNode, ValueNode,
    VariablePdaSeedNode,
};

fn main_program() -> ProgramNode {
    ProgramNode::new("main", "Main111111111111111111111111111111111111111")
        .add_defined_type(DefinedTypeNode::new("rate", FloatTypeNode::le(F64)))
        .add_pda(PdaNode::new(
            "ratePda",
            vec![VariablePdaSeedNode::new("rate", FloatTypeNode::le(F64)).into()],
        ))
        .add_pda(PdaNode::new(
            "linkedRatePda",
            vec![VariablePdaSeedNode::new("rate", DefinedTypeLinkNode::new("rate")).into()],
        ))
        .add_pda(PdaNode::new(
            "countPda",
            vec![VariablePdaSeedNode::new("count", IntegerTypeNode::le(U64)).into()],
        ))
}

fn other_program() -> ProgramNode {
    ProgramNode::new("other", "Other11111111111111111111111111111111111111")
        .add_defined_type(DefinedTypeNode::new("price", FloatTypeNode::le(F64)))
        .add_pda(PdaNode::new(
            "pricePda",
            vec![VariablePdaSeedNode::new("price", DefinedTypeLinkNode::new("price")).into()],
        ))
        .add_pda(PdaNode::new(
            "mainRatePda",
            vec![VariablePdaSeedNode::new("rate", DefinedTypeLinkNode::new("rate")).into()],
        ))
}

fn pda_value<T: Into<PdaSeedValueValue>>(pda: PdaValuePda, seed: &str, value: T) -> PdaValueNode {
    PdaValueNode::new(pda, vec![PdaSeedValueNode::new(seed, value)])
}

fn constant_seed_types() -> [fn(ValueNode) -> ConstantPdaSeedNode; 3] {
    [
        |value| ConstantPdaSeedNode::new(FloatTypeNode::le(F32), value),
        |value| ConstantPdaSeedNode::new(DefinedTypeLinkNode::new("rate"), value),
        |value| {
            let r#type =
                StringTypeNode::utf8().add_transform(HiddenPrefixTransformNode::new(vec![
                    constant_with_defined_type_link("rate", value),
                ]));
            ConstantPdaSeedNode::new(r#type, StringValueNode::new("x"))
        },
    ]
}

#[test]
fn constant_seeds_become_floats() {
    for seed in constant_seed_types() {
        let root = |value| {
            root_with_programs(
                main_program().add_pda(PdaNode::new("constantPda", vec![seed(value).into()])),
                other_program(),
            )
        };
        assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
    }
}

#[test]
fn constant_seeds_of_inline_pdas_become_floats() {
    for seed in constant_seed_types() {
        let root = |value| {
            let pda = PdaNode::new("inlinePda", vec![seed(value).into()]);
            let value = PdaValueNode::new(pda, vec![]);
            root_with_programs(
                main_program().add_instruction(instruction_with_account_default(value)),
                other_program(),
            )
        };
        assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
    }
}

#[test]
fn variable_seed_types_hold_constants_that_become_floats() {
    let root = |value| {
        let r#type = StringTypeNode::utf8().add_transform(HiddenPrefixTransformNode::new(vec![
            constant_with_defined_type_link("rate", value),
        ]));
        let pda = PdaNode::new(
            "namePda",
            vec![VariablePdaSeedNode::new("name", r#type).into()],
        );
        root_with_programs(main_program().add_pda(pda), other_program())
    };
    assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
}

#[test]
fn seed_values_of_account_defaults_become_floats() {
    for (pda, seed) in [
        (PdaValuePda::from(PdaLinkNode::new("ratePda")), "rate"),
        (PdaLinkNode::new("linkedRatePda").into(), "rate"),
        (pda_link_with_program("ratePda", "main").into(), "rate"),
        (pda_link_with_program("pricePda", "other").into(), "price"),
        (
            PdaNode::new(
                "inlinePda",
                vec![VariablePdaSeedNode::new("rate", DefinedTypeLinkNode::new("rate")).into()],
            )
            .into(),
            "rate",
        ),
    ] {
        let root = |value: ValueNode| {
            let value = pda_value(pda.clone(), seed, value);
            root_with_programs(
                main_program().add_instruction(instruction_with_account_default(value)),
                other_program(),
            )
        };
        assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
    }
}

#[test]
fn seed_values_of_provided_values_become_floats() {
    let root = |value: ValueNode| {
        let value = pda_value(PdaLinkNode::new("ratePda").into(), "rate", value);
        let instruction = InstructionNode {
            identifier: "update".into(),
            provides: vec![ProvidedNode::new("target", value)],
            ..InstructionNode::default()
        };
        root_with_programs(main_program().add_instruction(instruction), other_program())
    };
    assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
}

#[test]
fn pda_links_without_program_resolve_in_the_program_of_the_instruction() {
    let root = |value: ValueNode| {
        let value = pda_value(PdaLinkNode::new("pricePda").into(), "price", value);
        root_with_programs(
            main_program(),
            other_program().add_instruction(instruction_with_account_default(value)),
        )
    };
    assert_eq!(transform_integers_to_floats(root(integer())), root(float()));
}

#[test]
fn other_seed_values_keep_their_value() {
    for (pda, seed, value) in [
        (
            PdaValuePda::from(PdaLinkNode::new("missingPda")),
            "rate",
            PdaSeedValueValue::from(integer()),
        ),
        (
            pda_link_with_program("ratePda", "unknown").into(),
            "rate",
            integer().into(),
        ),
        (
            PdaLinkNode::new("pricePda").into(),
            "price",
            integer().into(),
        ),
        (
            pda_link_with_program("mainRatePda", "other").into(),
            "rate",
            integer().into(),
        ),
        (
            PdaLinkNode::new("ratePda").into(),
            "unknown",
            integer().into(),
        ),
        (
            PdaLinkNode::new("countPda").into(),
            "count",
            integer().into(),
        ),
        (
            PdaLinkNode::new("countPda").into(),
            "count",
            ValueNode::from(FloatValueNode::new(2.0)).into(),
        ),
        (
            PdaLinkNode::new("ratePda").into(),
            "rate",
            AccountValueNode::new("authority").into(),
        ),
        (
            PdaLinkNode::new("ratePda").into(),
            "rate",
            DataValueNode::new(path("rate")).into(),
        ),
    ] {
        let value = pda_value(pda, seed, value);
        let root = root_with_programs(
            main_program().add_instruction(instruction_with_account_default(value)),
            other_program(),
        );
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}

#[test]
fn constant_seeds_at_integer_types_keep_their_value() {
    for value in [integer(), FloatValueNode::new(2.0).into()] {
        let seed = ConstantPdaSeedNode::new(IntegerTypeNode::le(U64), value);
        let pda = PdaNode::new("constantPda", vec![seed.into()]);
        let root = root_with_programs(main_program().add_pda(pda), other_program());
        assert_eq!(transform_integers_to_floats(root.clone()), root);
    }
}
