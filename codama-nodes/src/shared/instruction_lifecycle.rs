use crate::InstructionLifecycle;

// The spec doesn't carry a notion of "default variant", so the
// generated `InstructionLifecycle` enum has no `#[derive(Default)]`.
// The default member is a Rust-side choice and lives here.
#[allow(clippy::derivable_impls)]
impl Default for InstructionLifecycle {
    fn default() -> Self {
        InstructionLifecycle::Live
    }
}
