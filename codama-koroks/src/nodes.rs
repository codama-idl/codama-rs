#[derive(Debug)]
pub struct RootNode {
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}

#[derive(Debug)]
pub struct ProgramNode {
    pub name: String,
    pub version: String,
    pub accounts: Vec<AccountNode>,
    pub instructions: Vec<InstructionNode>,
    pub defined_types: Vec<DefinedTypeNode>,
    pub pdas: Vec<PdaNode>,
}

#[derive(Debug)]
pub struct AccountNode {
    pub name: String,
}

#[derive(Debug)]
pub struct InstructionNode {
    pub name: String,
}

#[derive(Debug)]
pub struct DefinedTypeNode {
    pub name: String,
    pub ty: TypeNode,
}

#[derive(Debug)]
pub struct PdaNode {
    pub name: String,
}

#[derive(Debug)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
}

#[derive(Debug)]
pub struct NumberTypeNode {
    pub format: String,
}

#[derive(Debug)]
pub struct StringTypeNode {
    pub encoding: String,
}
