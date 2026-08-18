solana_program::declare_id!("MyProgramAddress1111111111111111111111111");

// Overrides the primary program's name, which would otherwise be derived from
// the crate name (`myCrateName`). The address still resolves from the manifest.
codama_program!(name = "myOverriddenProgramName");

#[derive(CodamaInstructions)]
pub enum MyInstruction {
    Create,
}
