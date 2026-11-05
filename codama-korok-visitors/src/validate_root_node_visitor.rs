use crate::KorokVisitor;
use codama_errors::{CodamaError, CodamaResult};
use codama_koroks::RootKorok;
use codama_nodes::{Node, NodePath, NodeVisitor, ValidateIdentifierCollisionVisitor};

/// Runs the node validators on the root node and fails with all their errors in one `CodamaError::InvalidRootNode`.
/// Plugins can add or rename nodes until the last one runs, so this check needs the final root node.
#[derive(Default)]
pub struct ValidateRootNodeVisitor;

impl ValidateRootNodeVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for ValidateRootNodeVisitor {
    fn visit_root(&mut self, korok: &mut RootKorok) -> CodamaResult<()> {
        let Some(Node::Root(root)) = &korok.node else {
            return Ok(());
        };
        let mut identifier_collisions = ValidateIdentifierCollisionVisitor::new();
        identifier_collisions.visit_root(root, &NodePath::default());
        let errors = identifier_collisions.errors;
        match errors.is_empty() {
            true => Ok(()),
            false => Err(CodamaError::InvalidRootNode(errors)),
        }
    }
}
