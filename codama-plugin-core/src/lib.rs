//! The [`KorokPlugin`] trait and the [`DefaultPlugin`].
//!
//! A plugin runs visitors on the korok tree at each phase of the generation, such as
//! [`KorokPlugin::on_fields_set`], and can resolve the directives that name it, such as
//! `type = my_plugin::custom`. The [`DefaultPlugin`] turns the Rust code and its attributes into nodes;
//! other plugins run alongside it to add or adjust nodes.

mod default_plugin;
mod directive_resolver;
mod plugin;
mod resolve_directives_visitor;

pub use default_plugin::*;
pub use directive_resolver::*;
pub use plugin::*;
pub use resolve_directives_visitor::*;
