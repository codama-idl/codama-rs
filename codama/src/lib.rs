//! Generate a [Codama](https://github.com/codama-idl/codama) IDL from the source code of your Solana programs.
//!
//! Programs describe themselves with the `#[codama(...)]` attribute and the `Codama*` derives, such as
//! [`CodamaAccount`] and [`CodamaInstruction`]. They expand to nothing, so a program depends on this crate
//! like any other. When compiling for Solana, this crate only exports these macros, to keep the rest of
//! Codama out of on-chain builds.
//!
//! Outside Solana, [`Codama`] loads crates, runs the plugins that turn their items into nodes, validates
//! the result and returns the IDL:
//!
//! ```
//! use codama::Codama;
//!
//! let program: proc_macro2::TokenStream = r#"
//!     codama_program!(identifier = "counter", address = "Counter111111111111111111111111111111111111");
//!
//!     #[derive(CodamaAccount)]
//!     pub struct Counter {
//!         pub count: u64,
//!     }
//! "#
//! .parse()
//! .unwrap();
//!
//! let idl = Codama::hydrate(program)?.get_idl()?;
//! assert_eq!(idl.program.accounts[0].identifier.as_ref(), "counter");
//! # Ok::<(), codama::CodamaError>(())
//! ```
//!
//! Use [`Codama::load`] to read a crate from disk instead. Plugins should depend on the crates whose types
//! they use, such as `codama-nodes` or `codama-attributes`, rather than on this one.

pub use codama_macros::*;

// Solana programs only need Codama macros, which expand to
// nothing at compile time. Everything else must be excluded
// to avoid compiling non-Solana-compatible code.

#[cfg(not(target_os = "solana"))]
mod codama;

#[cfg(not(target_os = "solana"))]
pub use {
    codama::*, codama_errors::*, codama_korok_visitors::*, codama_koroks::*, codama_nodes::*,
    codama_plugin_core::*, codama_stores::*, codama_validators::*, codama_visitors_core::*,
};
