use crate::ProgramNode;
use codama_nodes_derive::node;

#[node]
pub struct RootNode {
    // Data.
    standard: String,
    pub version: String,

    // Children.
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}

impl Default for RootNode {
    fn default() -> Self {
        Self {
            standard: "codama".into(),
            version: "1.0.0".into(), // TODO: Get this from Cargo.toml.
            program: Default::default(),
            additional_programs: Default::default(),
        }
    }
}

impl RootNode {
    pub fn new(program: ProgramNode) -> Self {
        Self {
            program,
            ..Self::default()
        }
    }

    pub fn add_program(mut self, program: ProgramNode) -> Self {
        self.additional_programs.push(program);
        self
    }
}

impl From<ProgramNode> for RootNode {
    fn from(program: ProgramNode) -> Self {
        Self::new(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        let node = RootNode::new(ProgramNode {
            name: "myProgram".into(),
            ..ProgramNode::default()
        });
        assert_eq!(
            node.program,
            ProgramNode {
                name: "myProgram".into(),
                ..ProgramNode::default()
            }
        );
        assert_eq!(node.additional_programs, vec![]);
    }

    #[test]
    fn from_program() {
        let node: RootNode = ProgramNode {
            name: "myProgram".into(),
            ..ProgramNode::default()
        }
        .into();
        assert_eq!(
            node.program,
            ProgramNode {
                name: "myProgram".into(),
                ..ProgramNode::default()
            }
        );
        assert_eq!(node.additional_programs, vec![]);
    }

    #[test]
    fn direct_instantiation() {
        let node = RootNode {
            standard: "codama".into(),
            version: "1.0.0".into(),
            program: ProgramNode::new("myProgram", "1111"),
            additional_programs: vec![
                ProgramNode::new("myProgramDependencyA", "2222"),
                ProgramNode::new("myProgramDependencyB", "3333"),
            ],
        };
        assert_eq!(node.program, ProgramNode::new("myProgram", "1111"));
        assert_eq!(
            node.additional_programs,
            vec![
                ProgramNode::new("myProgramDependencyA", "2222"),
                ProgramNode::new("myProgramDependencyB", "3333"),
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = RootNode::new(ProgramNode {
            name: "myProgram".into(),
            public_key: "1234..5678".into(),
            version: "1.2.3".into(),
            ..ProgramNode::default()
        });
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"rootNode","standard":"codama","version":"1.0.0","program":{"kind":"programNode","name":"myProgram","publicKey":"1234..5678","version":"1.2.3","accounts":[],"instructions":[],"definedTypes":[],"pdas":[],"errors":[]},"additionalPrograms":[]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"rootNode","standard":"codama","version":"1.0.0","program":{"kind":"programNode","name":"myProgram","publicKey":"1234..5678","version":"1.2.3","accounts":[],"instructions":[]},"additionalPrograms":[]}"#;
        let node: RootNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            RootNode::new(ProgramNode {
                name: "myProgram".into(),
                public_key: "1234..5678".into(),
                version: "1.2.3".into(),
                ..ProgramNode::default()
            })
        );
    }
}
