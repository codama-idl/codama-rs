use crate::{CombineTypesVisitor, KorokVisitor};
use codama_errors::CodamaResult;

pub struct SetDefinedTypesVisitor {
    combine_types: CombineTypesVisitor,
}

impl Default for SetDefinedTypesVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
        }
    }
}

impl SetDefinedTypesVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetDefinedTypesVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // Ensure the struct has the `CodamaType` attribute.
        if !korok.attributes.has_codama_derive("CodamaType") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the struct.
        self.combine_types.visit_struct(korok)?;

        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // Ensure the enum has the `CodamaType` attribute.
        if !korok.attributes.has_codama_derive("CodamaType") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the enum.
        self.combine_types.visit_enum(korok)?;

        Ok(())
    }
}
