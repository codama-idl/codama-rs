solana_program::declare_id!("MyProgramAddress1111111111111111111111111");

// Overrides the primary program's name, which would otherwise be derived from
// the crate name (`my_crate_name`). The address still resolves from the manifest.
codama_program!(identifier = "myOverriddenProgramName");

#[derive(CodamaInstructions)]
pub enum MyInstruction {
    Create,
}
