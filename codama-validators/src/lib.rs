//! Validate the nodes of a Codama tree, like `@codama/validators` in the JavaScript library.
//!
//! [`get_validation_items_visitor`] lists the [`ValidationItem`]s of a tree, such as missing identifiers,
//! identifier collisions or links to missing nodes, and [`throw_validator_items_visitor`] turns them into
//! an error from a given [`LogLevel`].

mod identifier_collisions;
mod throw_validator_items_visitor;
mod validation_item;
mod validation_items_visitor;

pub use throw_validator_items_visitor::*;
pub use validation_item::*;
pub use validation_items_visitor::*;
