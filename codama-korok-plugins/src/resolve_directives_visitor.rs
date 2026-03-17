use crate::DirectiveResolver;
use codama_attributes::{Attribute, CodamaDirective, Resolvable, SeedDirectiveType};
use codama_errors::CodamaResult;
use codama_korok_visitors::KorokVisitor;
use codama_koroks::*;
use codama_nodes::{InstructionInputValueNode, ValueNode};

/// A visitor that resolves all `Resolvable::Unresolved` entries in the korok tree
/// by delegating to the `DirectiveResolver`.
///
/// This visitor runs automatically as the first step in `resolve_plugins()`,
/// before any lifecycle hooks fire.
pub struct ResolveDirectivesVisitor<'a> {
    resolver: &'a dyn DirectiveResolver,
}

impl<'a> ResolveDirectivesVisitor<'a> {
    pub fn new(resolver: &'a dyn DirectiveResolver) -> Self {
        Self { resolver }
    }

    fn resolve_attributes(
        &self,
        attributes: &mut codama_attributes::Attributes,
    ) -> CodamaResult<()> {
        for attr in attributes.iter_mut() {
            let Attribute::Codama(codama_attr) = attr else {
                continue;
            };
            self.resolve_directive(codama_attr.directive.as_mut())?;
        }
        Ok(())
    }

    fn resolve_directive(&self, directive: &mut CodamaDirective) -> CodamaResult<()> {
        match directive {
            CodamaDirective::Type(d) => {
                self.resolve_type(&mut d.node)?;
            }
            CodamaDirective::DefaultValue(d) => {
                self.resolve_instruction_input_value(&mut d.node)?;
            }
            CodamaDirective::Account(d) => {
                if let Some(ref mut dv) = d.default_value {
                    self.resolve_instruction_input_value(dv)?;
                }
            }
            CodamaDirective::Field(d) => {
                self.resolve_type_node(&mut d.r#type)?;
                if let Some(ref mut dv) = d.default_value {
                    self.resolve_value_node(dv)?;
                }
            }
            CodamaDirective::Argument(d) => {
                self.resolve_type_node(&mut d.r#type)?;
                if let Some(ref mut dv) = d.default_value {
                    self.resolve_instruction_input_value(dv)?;
                }
            }
            CodamaDirective::Seed(d) => match &mut d.seed {
                SeedDirectiveType::Variable { r#type, .. } => {
                    self.resolve_type_node(r#type)?;
                }
                SeedDirectiveType::Constant { r#type, value } => {
                    self.resolve_type_node(r#type)?;
                    self.resolve_value_node(value)?;
                }
                SeedDirectiveType::Linked(_) => {}
            },
            // Other directives don't contain resolvable slots.
            _ => {}
        }
        Ok(())
    }

    fn resolve_type(
        &self,
        resolvable: &mut Resolvable<codama_nodes::RegisteredTypeNode>,
    ) -> CodamaResult<()> {
        if let Resolvable::Unresolved(directive) = resolvable {
            let resolved = self.resolver.resolve_type_directive(directive)?;
            *resolvable = Resolvable::Resolved(resolved);
        }
        Ok(())
    }

    fn resolve_type_node(
        &self,
        resolvable: &mut Resolvable<codama_nodes::TypeNode>,
    ) -> CodamaResult<()> {
        if let Resolvable::Unresolved(directive) = resolvable {
            let registered = self.resolver.resolve_type_directive(directive)?;
            let type_node = codama_nodes::TypeNode::try_from(registered)?;
            *resolvable = Resolvable::Resolved(type_node);
        }
        Ok(())
    }

    fn resolve_instruction_input_value(
        &self,
        resolvable: &mut Resolvable<InstructionInputValueNode>,
    ) -> CodamaResult<()> {
        if let Resolvable::Unresolved(directive) = resolvable {
            let resolved = self.resolver.resolve_value_directive(directive)?;
            *resolvable = Resolvable::Resolved(resolved);
        }
        Ok(())
    }

    fn resolve_value_node(&self, resolvable: &mut Resolvable<ValueNode>) -> CodamaResult<()> {
        if let Resolvable::Unresolved(directive) = resolvable {
            let instruction_input = self.resolver.resolve_value_directive(directive)?;
            let value_node = ValueNode::try_from(instruction_input)?;
            *resolvable = Resolvable::Resolved(value_node);
        }
        Ok(())
    }
}

impl KorokVisitor for ResolveDirectivesVisitor<'_> {
    fn visit_root(&mut self, korok: &mut RootKorok) -> CodamaResult<()> {
        self.visit_children(korok)
    }

    fn visit_crate(&mut self, korok: &mut CrateKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)?;
        self.visit_children(korok)
    }

    fn visit_file_module(&mut self, korok: &mut FileModuleKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)?;
        self.visit_children(korok)
    }

    fn visit_module(&mut self, korok: &mut ModuleKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)?;
        self.visit_children(korok)
    }

    fn visit_struct(&mut self, korok: &mut StructKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)?;
        self.visit_children(korok)
    }

    fn visit_enum(&mut self, korok: &mut EnumKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)?;
        self.visit_children(korok)
    }

    fn visit_enum_variant(&mut self, korok: &mut EnumVariantKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)?;
        self.visit_children(korok)
    }

    fn visit_field(&mut self, korok: &mut FieldKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)
    }

    fn visit_unsupported_item(&mut self, korok: &mut UnsupportedItemKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)
    }

    fn visit_impl(&mut self, korok: &mut ImplKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)?;
        self.visit_children(korok)
    }

    fn visit_const(&mut self, korok: &mut ConstKorok) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)
    }

    fn visit_unsupported_impl_item(
        &mut self,
        korok: &mut UnsupportedImplItemKorok,
    ) -> CodamaResult<()> {
        self.resolve_attributes(&mut korok.attributes)
    }
}
