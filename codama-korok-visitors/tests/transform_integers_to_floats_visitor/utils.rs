use codama_korok_visitors::{KorokVisitable, TransformIntegersToFloatsVisitor};
use codama_koroks::RootKorok;
use codama_nodes::test_utils::id;
use codama_nodes::{
    AccountNode, ConstantValueNode, DefinedTypeLinkNode, DefinedTypeNode, FloatFormat::F64,
    FloatTypeNode, FloatValueNode, InstructionAccountNode, InstructionNode, IntegerValueNode, Node,
    PdaLinkNode, PdaValueNode, ProgramLinkNode, ProgramNode, RootNode, StructFieldTypeNode,
    StructTypeNode, TypeNode, ValueNode,
};
use codama_stores::RootStore;

pub fn transform_integers_to_floats(root: RootNode) -> RootNode {
    let store = RootStore { crates: vec![] };
    let mut korok = RootKorok {
        crates: vec![],
        node: Some(root.into()),
        store: &store,
    };
    korok
        .accept(&mut TransformIntegersToFloatsVisitor::new())
        .unwrap();
    let Some(Node::Root(root)) = korok.node else {
        panic!("Expected a root node");
    };
    root
}

pub fn integer() -> ValueNode {
    IntegerValueNode::new(1u64).into()
}

pub fn float() -> ValueNode {
    FloatValueNode::new(1.0).into()
}

pub fn field<T: Into<TypeNode>>(r#type: T, value: ValueNode) -> StructFieldTypeNode {
    StructFieldTypeNode {
        default_value: Box::new(Some(value)),
        ..StructFieldTypeNode::new(id("value"), r#type)
    }
}

pub fn account_with_field<T: Into<TypeNode>>(r#type: T, value: ValueNode) -> AccountNode {
    AccountNode::new(
        id("config"),
        StructTypeNode::new(vec![field(r#type, value)]),
    )
}

pub fn account_with_type<T: Into<TypeNode>>(r#type: T) -> AccountNode {
    AccountNode::new(
        id("config"),
        StructTypeNode::new(vec![StructFieldTypeNode::new(id("value"), r#type)]),
    )
}

pub fn root_with_account(program: ProgramNode, account: AccountNode) -> RootNode {
    RootNode::new(program.add_account(account))
}

pub fn root_with_programs(main: ProgramNode, other: ProgramNode) -> RootNode {
    RootNode::new(main).add_program(other)
}

pub fn program_with_float_defined_type(identifier: &str) -> ProgramNode {
    ProgramNode::default()
        .add_defined_type(DefinedTypeNode::new(id(identifier), FloatTypeNode::le(F64)))
}

pub fn constant_with_defined_type_link(identifier: &str, value: ValueNode) -> ConstantValueNode {
    ConstantValueNode::new(DefinedTypeLinkNode::new(id(identifier)), value)
}

pub fn type_link_with_program(identifier: &str, program: &str) -> DefinedTypeLinkNode {
    DefinedTypeLinkNode::new_from_program(id(identifier), ProgramLinkNode::new(id(program)))
}

pub fn pda_link_with_program(identifier: &str, program: &str) -> PdaLinkNode {
    PdaLinkNode::new_from_program(id(identifier), ProgramLinkNode::new(id(program)))
}

pub fn instruction_with_account_default(value: PdaValueNode) -> InstructionNode {
    InstructionNode {
        identifier: id("update"),
        accounts: vec![InstructionAccountNode {
            default_value: Box::new(Some(value.into())),
            ..InstructionAccountNode::new(id("target"), true, false)
        }],
        ..InstructionNode::default()
    }
}
