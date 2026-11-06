use crate::{parse_pda_node, CombineTypesVisitor, KorokVisitor};
use codama_attributes::{
    Attributes, DiscriminatorDirective, EnumDiscriminatorDirective, PdaDirective, ProgramDirective,
    TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::FieldKorok;
use codama_nodes::{
    AccountNode, CamelCaseString, DefaultValueStrategy, EnumVariantTypeNode,
    FieldDiscriminatorNode, IntegerValueNode, Node, PdaLinkNode, ProgramNode, StructFieldTypeNode,
    StructTypeNode, TypeNode,
};
use codama_syn_helpers::extensions::*;

pub struct SetAccountsVisitor {
    combine_types: CombineTypesVisitor,
    enum_name: String,
    enum_discriminator: EnumDiscriminatorDirective,
    enum_current_discriminator: usize,
}

impl Default for SetAccountsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
            enum_name: "".to_string(),
            enum_discriminator: EnumDiscriminatorDirective::default(),
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
        let (name, data) = parse_struct(korok)?;
        let account = AccountNode {
            discriminators: DiscriminatorDirective::nodes(&korok.attributes),
            pda: parse_pda_link_node(&korok.attributes),
            ..AccountNode::new(name, data)
        };

        let node = wrap_account_in_program_node_when_seeds_are_defined(
            account,
            &korok.attributes,
            &korok.fields,
        )?;
        korok.node = Some(ProgramDirective::apply(&korok.attributes, node));
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

        // Get enum discriminator info.
        let enum_discriminator = korok
            .attributes
            .get_last(EnumDiscriminatorDirective::filter)
            .cloned()
            .unwrap_or_else(|| EnumDiscriminatorDirective::from(&korok.node));

        // Transform each variant into an `AccountNode`.
        self.enum_name = korok.ast.ident.to_string();
        self.enum_discriminator = enum_discriminator;
        self.enum_current_discriminator = 0;
        self.visit_children(korok)?;

        // Gather all accounts in the variants.
        let accounts = korok
            .variants
            .iter()
            .flat_map(|variant| match &variant.node {
                Some(Node::Account(account)) => vec![account.clone()],
                Some(Node::Program(program)) => program.accounts.clone(),
                _ => vec![],
            })
            .collect::<Vec<_>>();

        // Gather all PDAs the variants.
        let pdas = korok
            .variants
            .iter()
            .flat_map(|variant| match &variant.node {
                Some(Node::Pda(pda)) => vec![pda.clone()],
                Some(Node::Program(program)) => program.pdas.clone(),
                _ => vec![],
            })
            .collect::<Vec<_>>();

        let node: Node = ProgramNode {
            accounts,
            pdas,
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
        // Update current discriminator.
        let current_discriminator = match &korok.ast.discriminant {
            Some((_, expr)) => expr.as_unsigned_integer()?,
            _ => self.enum_current_discriminator,
        };
        self.enum_current_discriminator = current_discriminator + 1;

        // Skip variants with #[codama(skip)] directive.
        if korok.attributes.has_codama_attribute("skip") {
            return Ok(());
        };

        let discriminator = StructFieldTypeNode {
            default_value_strategy: Some(DefaultValueStrategy::Omitted),
            default_value: Box::new(Some(
                IntegerValueNode::new(current_discriminator as u64).into(),
            )),
            ..StructFieldTypeNode::from(&self.enum_discriminator)
        };
        let discriminator_node = FieldDiscriminatorNode::try_from(&self.enum_discriminator)?;
        let (name, mut data) = parse_enum_variant(korok, &self.enum_name)?;
        data.fields.insert(0, discriminator);

        let mut discriminators = DiscriminatorDirective::nodes(&korok.attributes);
        discriminators.insert(0, discriminator_node.into());

        let account = AccountNode {
            discriminators,
            pda: parse_pda_link_node(&korok.attributes),
            ..AccountNode::new(name, data)
        };

        korok.node = Some(wrap_account_in_program_node_when_seeds_are_defined(
            account,
            &korok.attributes,
            &korok.fields,
        )?);
        Ok(())
    }
}

fn parse_struct(
    korok: &codama_koroks::StructKorok,
) -> CodamaResult<(CamelCaseString, StructTypeNode)> {
    // Ensure we have a `DefinedTypeNode` to work with.
    if let Some(Node::DefinedType(node)) = &korok.node {
        // Ensure the data type is a struct.
        if let TypeNode::Struct(data) = (*node.r#type).clone() {
            return Ok((node.identifier.clone(), data));
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" struct could not be used as an Account because its type is not a `StructTypeNode`.",
        korok.ast.ident,
    );
    Err(korok.ast.error(message).into())
}

fn parse_enum_variant(
    korok: &codama_koroks::EnumVariantKorok,
    enum_name: &str,
) -> CodamaResult<(CamelCaseString, StructTypeNode)> {
    // Ensure we have a `Node`.
    if let Some(node) = &korok.node {
        // Ensure we have a `EnumVariantTypeNode`.
        if let Ok(node) = EnumVariantTypeNode::try_from(node.clone()) {
            match *node.data {
                Some(TypeNode::Struct(data)) => return Ok((node.identifier, data)),
                // Or an empty variant.
                None => return Ok((node.identifier, StructTypeNode::new(vec![]))),
                _ => {}
            }
        };
    };

    // Handle error.
    let message = format!(
        "The \"{}\" variant of the \"{enum_name}\" enum could not be used as an Account because we cannot get a `StructTypeNode` for it. This is likely because it is not using named fields.",
        korok.ast.ident
    );
    Err(korok.ast.error(message).into())
}

fn parse_pda_link_node(attributes: &Attributes) -> Option<PdaLinkNode> {
    attributes
        .get_last(PdaDirective::filter)
        .map(|directive| directive.pda.clone())
}

fn wrap_account_in_program_node_when_seeds_are_defined(
    account: AccountNode,
    attributes: &Attributes,
    fields: &[FieldKorok],
) -> CodamaResult<Node> {
    // Return the AccountNode directly if there are no seed directives.
    if !attributes.has_codama_attribute("seed") {
        return Ok(account.into());
    }

    // Create a PDA node from the seed directives.
    let pda_name = match &account.pda {
        Some(pda_link) => pda_link.identifier.clone(),
        None => account.identifier.clone(),
    };
    let pda = parse_pda_node(pda_name, attributes, fields)?;

    // Add both the account and the linked PDA to a program node.
    Ok(ProgramNode {
        accounts: vec![AccountNode {
            pda: account
                .pda
                .or(Some(PdaLinkNode::new(pda.identifier.clone()))),
            ..account
        }],
        pdas: vec![pda],
        ..ProgramNode::default()
    }
    .into())
}
