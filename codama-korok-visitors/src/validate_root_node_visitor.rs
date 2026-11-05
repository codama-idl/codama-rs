use crate::KorokVisitor;
use codama_errors::{CodamaError, CodamaResult};
use codama_koroks::RootKorok;
use codama_nodes::{
    Linkables, Node, NodePath, NodeVisitor, ValidateIdentifierCollisionVisitor,
    ValidateLinksVisitor, ValidatePathsVisitor,
};

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
        let path = NodePath::default();

        let mut identifier_collisions = ValidateIdentifierCollisionVisitor::new();
        identifier_collisions.visit_root(root, &path);

        let linkables = Linkables::new(root);
        let mut links = ValidateLinksVisitor::new(&linkables);
        links.visit_root(root, &path);

        let mut paths = ValidatePathsVisitor::new(&linkables);
        paths.visit_root(root, &path);

        let errors = [identifier_collisions.errors, links.errors, paths.errors].concat();
        match errors.is_empty() {
            true => Ok(()),
            false => Err(CodamaError::InvalidRootNode(errors)),
        }
    }
}
