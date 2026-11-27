//! Load crates and their files.
//!
//! A [`RootStore`] loads one or more crates, each as a [`CrateStore`] holding its manifest and the parsed
//! files of its modules as [`FileModuleStore`]s, so the korok tree of `codama-koroks` can borrow them.

mod crate_store;
mod file_module_store;
mod root_store;

pub use crate_store::*;
pub use file_module_store::*;
pub use root_store::*;
