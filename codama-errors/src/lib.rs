//! The error and result types shared by the Codama crates.
//!
//! Every fallible function of Codama returns a [`CodamaResult`]. A [`CodamaError`] converts into a
//! `compile_error!` with [`CodamaError::to_compile_error`], which the macros use to report invalid
//! attributes, and the [`CombineErrors`] traits gather several errors into one, so a single run reports
//! every problem.

mod combine_errors;
mod errors;

pub use combine_errors::*;
pub use errors::*;
