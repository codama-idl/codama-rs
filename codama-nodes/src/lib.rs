#![warn(missing_copy_implementations)]

mod count_nodes;
mod discriminator_nodes;
mod display_nodes;
mod generated;
mod link_nodes;
mod node;
mod pda_seed_nodes;
mod shared;
mod traits;
mod transform_nodes;
mod type_nodes;
mod value_nodes;

// Kept off until this node family is ported to spec v2, so the crate compiles in the meantime.
// mod account_node;
// mod constant_node;
// mod contextual_value_nodes;
// mod defined_type_node;
// mod error_node;
// mod event_node;
// mod instruction_account_node;
// mod instruction_byte_delta_node;
// mod instruction_node;
// mod instruction_remaining_accounts_node;
// mod instruction_status_node;
// mod pda_node;
// mod plugin_node;
// mod program_node;
// mod provided_node;
// mod root_node;

pub use generated::*;
pub use node::*;
pub use shared::*;
pub use traits::*;
pub use type_nodes::*;

// Serde helper function to use with `#[serde(some_thing = "crate::is_default")]`.
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
