use crate::OptionalAccountStrategy;

// The spec doesn't carry a notion of "default variant", so the
// generated `OptionalAccountStrategy` enum has no `#[derive(Default)]`.
// The default member is a Rust-side choice and lives here.
#[allow(clippy::derivable_impls)]
impl Default for OptionalAccountStrategy {
    fn default() -> Self {
        OptionalAccountStrategy::ProgramId
    }
}
