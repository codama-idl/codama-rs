pub use codama_macros::*;

// Solana programs only need Codama macros, which expand to
// nothing at compile time. Everything else must be excluded
// to avoid compiling non-Solana-compatible code.

#[cfg(not(target_os = "solana"))]
mod codama;

#[cfg(not(target_os = "solana"))]
pub use {
    codama::*, codama_errors::*, codama_korok_plugins::*, codama_korok_visitors::*,
    codama_koroks::*, codama_nodes::*, codama_stores::*,
};
