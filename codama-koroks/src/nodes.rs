pub struct RootNode {
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}

pub struct ProgramNode {
    pub name: String,
    pub version: String,
    pub accounts: Vec<AccountNode>,
    pub instructions: Vec<InstructionNode>,
    pub defined_types: Vec<DefinedTypeNode>,
    pub pdas: Vec<PdaNode>,
}

pub struct AccountNode {
    pub name: String,
}

pub struct InstructionNode {
    pub name: String,
}

pub struct DefinedTypeNode {
    pub name: String,
    pub ty: TypeNode,
}

pub struct PdaNode {
    pub name: String,
}

pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
}

pub struct NumberTypeNode {
    pub format: String,
}

pub struct StringTypeNode {
    pub encoding: String,
}
