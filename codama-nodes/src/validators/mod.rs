//! Node visitors that check the final root node. `ValidateRootNodeVisitor` runs them after all plugins.

mod validate_identifier_collision_visitor;
mod validate_links_visitor;
mod validate_paths_visitor;

pub use validate_identifier_collision_visitor::*;
pub use validate_links_visitor::*;
pub use validate_paths_visitor::*;
