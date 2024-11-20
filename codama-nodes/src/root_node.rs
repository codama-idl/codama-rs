use crate::ProgramNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone, Default)]
pub struct RootNode {
    // Children.
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}

impl RootNode {
    pub fn new(program: ProgramNode) -> Self {
        Self {
            program,
            ..Self::default()
        }
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
            program: ProgramNode {
                name: "myProgram".into(),
                ..ProgramNode::default()
            },
            additional_programs: vec![
                ProgramNode {
                    name: "myProgramDependencyA".into(),
                    ..ProgramNode::default()
                },
                ProgramNode {
                    name: "myProgramDependencyB".into(),
                    ..ProgramNode::default()
                },
            ],
        };
        assert_eq!(
            node.program,
            ProgramNode {
                name: "myProgram".into(),
                ..ProgramNode::default()
            }
        );
        assert_eq!(
            node.additional_programs,
            vec![
                ProgramNode {
                    name: "myProgramDependencyA".into(),
                    ..ProgramNode::default()
                },
                ProgramNode {
                    name: "myProgramDependencyB".into(),
                    ..ProgramNode::default()
                },
            ]
        );
    }
}
