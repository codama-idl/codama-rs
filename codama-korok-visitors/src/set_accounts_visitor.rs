use crate::{CombineTypesVisitor, KorokVisitor};
use codama_errors::CodamaResult;
use codama_nodes::{
    AccountNode, DefaultValueStrategy, Docs, EnumVariantTypeNode, FieldDiscriminatorNode,
    NestedTypeNode, NestedTypeNodeTrait, Node, NumberFormat::U8, NumberTypeNode, NumberValueNode,
    ProgramNode, StructFieldTypeNode, StructTypeNode,
};
use codama_syn_helpers::extensions::*;

pub struct SetAccountsVisitor {
    combine_types: CombineTypesVisitor,
    enum_name: Option<String>,
    enum_current_discriminator: usize,
}

impl Default for SetAccountsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
            enum_name: None,
            enum_current_discriminator: 0,
        }
    }
}

impl SetAccountsVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetAccountsVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the struct has the `CodamaAccount` attribute.
        if !korok.attributes.has_codama_derive("CodamaAccount") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the struct, if it doesn't already exist.
        self.combine_types.visit_struct(korok)?;

        // Transform the defined type into an account node.
        let data = get_nested_struct_type_node_from_struct(korok)?;
        korok.node = Some(
            AccountNode {
                name: korok.ast.ident.to_string().into(),
                size: None,
                docs: Docs::default(),
                data,
                pda: None,
                discriminators: vec![],
            }
            .into(),
        );

        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the struct has the `CodamaAccounts` attribute.
        if !korok.attributes.has_codama_derive("CodamaAccounts") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the enum.
        self.combine_types.visit_enum(korok)?;

        // Transform each variant into an `AccountNode`.
        self.enum_name = Some(korok.ast.ident.to_string());
        self.enum_current_discriminator = 0;
        self.visit_children(korok)?;
        self.enum_name = None;
        self.enum_current_discriminator = 0;

        // Gather all accounts in a `ProgramNode`.
        let accounts = korok
            .variants
            .iter()
            .filter_map(|variant| match &variant.node {
                Some(Node::Account(account)) => Some(account.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        korok.node = Some(
            ProgramNode {
                accounts,
                ..ProgramNode::default()
            }
            .into(),
        );

        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        // Update current discriminator.
        let current_discriminator = self.enum_current_discriminator;
        self.enum_current_discriminator = match &korok.ast.discriminant {
            Some((_, expr)) => expr.as_literal_integer()?,
            _ => current_discriminator + 1,
        };

        let discriminator_name = "discriminator".to_string(); // TODO: Offer a directive to customize this.
        let discriminator = StructFieldTypeNode {
            name: discriminator_name.clone().into(),
            default_value_strategy: Some(DefaultValueStrategy::Omitted),
            docs: Docs::default(),
            r#type: NumberTypeNode::le(U8).into(),
            default_value: Some(NumberValueNode::new(current_discriminator as u64).into()),
        };
        let data = get_nested_struct_type_node_from_enum_variant(korok, &self.enum_name)?
            .map_nested_type_node(|node| {
                let mut fields = node.fields;
                fields.insert(0, discriminator);
                StructTypeNode { fields, ..node }
            });
        let discriminator_node = FieldDiscriminatorNode::new(discriminator_name, 0);

        korok.node = Some(
            AccountNode {
                name: korok.ast.ident.to_string().into(),
                size: None,
                docs: Docs::default(),
                data,
                pda: None,
                discriminators: vec![discriminator_node.into()],
            }
            .into(),
        );

        Ok(())
    }
}

fn get_nested_struct_type_node_from_struct(
    korok: &codama_koroks::StructKorok,
) -> CodamaResult<NestedTypeNode<StructTypeNode>> {
    // Ensure we have a `DefinedTypeNode` to work with.
    if let Some(Node::DefinedType(node)) = &korok.node {
        // Ensure the data type is a struct.
        if let Ok(data) = NestedTypeNode::<StructTypeNode>::try_from(node.r#type.clone()) {
            return Ok(data);
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" struct could not be used as an Account because its type is not a `NestedTypeNode<StructTypeNode>`.",
        korok.ast.ident.to_string(),
    );
    Err(korok.ast.error(message).into())
}

fn get_nested_struct_type_node_from_enum_variant(
    korok: &codama_koroks::EnumVariantKorok,
    enum_name: &Option<String>,
) -> CodamaResult<NestedTypeNode<StructTypeNode>> {
    // Ensure we have a `Node`.
    if let Some(node) = &korok.node {
        // Ensure we have a `EnumVariantTypeNode`.
        if let Ok(node) = EnumVariantTypeNode::try_from(node.clone()) {
            match node {
                // Ensure we have a non-nested `StructTypeNode`.
                EnumVariantTypeNode::Struct(node) => {
                    return Ok(node.r#struct);
                }
                // Or an empty variant.
                EnumVariantTypeNode::Empty(_) => return Ok(StructTypeNode::new(vec![]).into()),
                _ => {}
            }
        };
    };

    // Handle error.
    let message_prefix = match enum_name {
        Some(name) => format!(
            "The \"{}\" variant of the \"{}\" enum",
            korok.ast.ident, name
        ),
        None => format!("The \"{}\" variant", korok.ast.ident),
    };
    let message = format!(
        "{} could not be used as an Account because we cannot get a `StructTypeNode` for it. This is likely because it is not using nammed fields.",
        message_prefix
    );
    Err(korok.ast.error(message).into())
}
