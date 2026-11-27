use crate::{
    enum_items::{parse_struct_data, parse_variant_data, EnumItemContext, ItemKind},
    walk_enum, CombineTypesVisitor, KorokVisitor,
};
use codama_attributes::{
    DiscriminatorDirective, EnumDiscriminatorDirective, ProgramDirective, SkipDirective,
    TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_nodes::{EventNode, Node, ProgramNode};

pub struct SetEventsVisitor {
    combine_types: CombineTypesVisitor,
    /// The enum being visited, if any.
    r#enum: EnumItemContext,
}

impl Default for SetEventsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
            r#enum: EnumItemContext::default(),
        }
    }
}

impl SetEventsVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetEventsVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the struct has the `CodamaEvent` attribute.
        if !korok.attributes.has_codama_derive("CodamaEvent") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the struct, if it doesn't already exist.
        self.combine_types.visit_struct(korok)?;

        // Transform the defined type into an event node.
        let (name, data) = parse_struct_data(korok, ItemKind::Event)?;

        let event = EventNode {
            discriminators: DiscriminatorDirective::nodes(&korok.attributes),
            ..EventNode::new(name, data)
        };

        korok.node = Some(ProgramDirective::apply(&korok.attributes, event.into()));

        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the enum has the `CodamaEvents` attribute.
        if !korok.attributes.has_codama_derive("CodamaEvents") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the enum.
        self.combine_types.visit_enum(korok)?;

        // Get enum discriminator info.
        let enum_discriminator = korok
            .attributes
            .get_last(EnumDiscriminatorDirective::filter)
            .cloned()
            .unwrap_or_else(|| EnumDiscriminatorDirective::from(&korok.node));

        // Transform each variant into an `EventNode`.
        self.r#enum = EnumItemContext::new(korok.ast.ident.to_string(), enum_discriminator);
        walk_enum(self, korok)?;

        // Gather all events in the variants.
        let events = korok
            .variants
            .iter()
            .filter_map(|variant| match &variant.node {
                Some(Node::Event(event)) => Some(event.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        let node: Node = ProgramNode {
            events,
            ..ProgramNode::default()
        }
        .into();

        korok.node = Some(ProgramDirective::apply(&korok.attributes, node));

        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        let current_discriminator = self.r#enum.next_discriminator(korok)?;

        // Skip variants with #[codama(skip)] directive.
        if korok.attributes.has(SkipDirective::filter) {
            return Ok(());
        };

        let discriminator = self.r#enum.discriminator_field(current_discriminator);
        let (name, mut data) = parse_variant_data(korok, &self.r#enum, ItemKind::Event)?;

        data.fields.insert(0, discriminator);

        let mut discriminators = DiscriminatorDirective::nodes(&korok.attributes);
        discriminators.insert(0, self.r#enum.discriminator_node().into());

        let event = EventNode {
            discriminators,
            ..EventNode::new(name, data)
        };

        korok.node = Some(event.into());

        Ok(())
    }
}
