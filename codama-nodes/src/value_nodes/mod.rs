mod array_value_node;
mod boolean_value_node;
mod bytes_value_node;
mod constant_value_node;
mod enum_value_node;
mod injected_value_node;
mod map_entry_value_node;
mod map_value_node;
mod none_value_node;
mod number_value_node;
mod public_key_value_node;
mod set_value_node;
mod some_value_node;
mod string_value_node;
mod struct_field_value_node;
mod struct_value_node;
mod tuple_value_node;
mod value_node;

// `number_value_node.rs` keeps the bespoke `Number` enum (with its
// custom `serde(from/into = "JsonNumber")` + the 8 `From<uN/iN/fN>`
// impls); the rest of the module is constructors + tests. Re-export
// so `crate::Number` resolves.
pub use number_value_node::*;
