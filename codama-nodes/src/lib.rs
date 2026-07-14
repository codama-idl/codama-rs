#![warn(missing_copy_implementations)]

mod account_node;
mod constant_node;
mod contextual_value_nodes;
mod count_nodes;
mod defined_type_node;
mod discriminator_nodes;
mod display_nodes;
mod error_node;
mod event_node;
mod generated;
mod instruction_account_node;
mod instruction_argument_node;
mod instruction_byte_delta_node;
mod instruction_node;
mod instruction_remaining_accounts_node;
mod instruction_status_node;
mod link_nodes;
mod node;
mod pda_node;
mod pda_seed_nodes;
mod plugin_node;
mod program_node;
mod provided_node;
mod root_node;
mod shared;
mod traits;
mod type_nodes;
mod value_nodes;

pub use generated::*;
pub use node::*;
pub use shared::*;
pub use traits::*;
pub use type_nodes::*;
pub use value_nodes::*;

// Serde helper function to use with `#[serde(some_thing = "crate::is_default")]`.
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
