use crate::{
    AccountLinkNode, DefinedTypeNode, Identifier, Linkables, Path, PathKey, ProgramLinkNode,
    ProgramScope, PublicKeyTypeNode, StructFieldTypeNode, TypeNode,
};

/// Whether a `.data` segment on a public key with an `accountLink` continues into the data of the linked account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountLinkResolution {
    /// Contextual values read account state, so `mint.data.decimals` reads the `decimals` of the linked account.
    Allow,
    /// Field discriminators only read the bytes they identify, so such a segment is `PathError::ForbiddenAccountLink`.
    Forbid,
}

/// The node that a path expression resolves to. A struct field is not resolved to its type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypePathNode<'a> {
    Field(&'a StructFieldTypeNode),
    /// A tuple, array or set item, or the data of a linked account.
    Type(&'a TypeNode),
}

/// Why a path expression does not resolve.
#[derive(Debug, Clone, PartialEq)]
pub enum PathError<'a> {
    /// The segment does not apply to the type it reaches, such as a missing struct field or a tuple index out of range.
    /// An alias cycle of defined types reaches no type, so `reached` is then the link that closes the cycle.
    Unresolvable {
        segment: String,
        reached: &'a TypeNode,
    },
    /// A `.data` segment on a public key with an `accountLink`, with `AccountLinkResolution::Forbid`.
    ForbiddenAccountLink,
    /// A defined type link or an account link on the way points to no node.
    DanglingLink,
}

/// Resolves `path` against `data`, a type of `program`, and returns the node that the path resolves to.
/// A port of the JS `resolveTypePath`. A `.identifier` segment resolves to the struct field with that identifier.
/// An `[n]` segment resolves to a tuple item, or to the item of an array or set with no check against the count.
/// Defined type links resolve to their types, in the program that holds each link.
/// Option, map and enum types are never entered, so a segment on them is `PathError::Unresolvable`.
pub fn resolve_type_path<'a>(
    linkables: &Linkables<'a>,
    program: &'a Identifier,
    data: &'a TypeNode,
    path: &Path,
    account_link_resolution: AccountLinkResolution,
) -> Result<TypePathNode<'a>, PathError<'a>> {
    let mut resolver = Resolver {
        linkables,
        program,
        account_link_resolution,
    };
    let mut node = TypePathNode::Type(data);
    for segment in path.segments() {
        node = resolver.resolve_segment(node, segment)?;
    }
    Ok(node)
}

/// The state of one path resolution.
struct Resolver<'l, 't> {
    linkables: &'l Linkables<'t>,
    /// The program of the current node. Unqualified links resolve in it, and a resolved link moves it.
    program: &'t Identifier,
    account_link_resolution: AccountLinkResolution,
}

impl<'l, 't> Resolver<'l, 't> {
    /// The node that `segment` resolves to from `node`.
    fn resolve_segment(
        &mut self,
        node: TypePathNode<'t>,
        segment: PathKey<'_>,
    ) -> Result<TypePathNode<'t>, PathError<'t>> {
        let type_node = self.resolve_defined_type_links(node.type_node(), segment)?;
        let next = match (segment, type_node) {
            (
                PathKey::Field("data"),
                TypeNode::PublicKey(PublicKeyTypeNode {
                    account_link: Some(account_link),
                    ..
                }),
            ) => return self.resolve_account_link(account_link),
            (PathKey::Field(identifier), TypeNode::Struct(struct_type)) => struct_type
                .fields
                .iter()
                .find(|field| field.identifier.as_str() == identifier)
                .map(TypePathNode::Field),
            (PathKey::Index(index), TypeNode::Tuple(tuple)) => {
                tuple.items.get(index).map(TypePathNode::Type)
            }
            (PathKey::Index(_), TypeNode::Array(array)) => Some(TypePathNode::Type(&array.item)),
            (PathKey::Index(_), TypeNode::Set(set)) => Some(TypePathNode::Type(&set.item)),
            _ => None,
        };
        next.ok_or_else(|| unresolvable(segment, type_node))
    }

    /// Resolves defined type links from `node` to the type they name.
    /// The alias cycle guard is per segment, as a later segment may resolve the same link again.
    fn resolve_defined_type_links(
        &mut self,
        mut node: &'t TypeNode,
        segment: PathKey<'_>,
    ) -> Result<&'t TypeNode, PathError<'t>> {
        let mut resolved_types: Vec<&DefinedTypeNode> = Vec::new();
        while let TypeNode::Link(link) = node {
            let program = self.linked_program(link.program.as_ref())?;
            let defined_type = program
                .defined_type(&link.identifier)
                .ok_or(PathError::DanglingLink)?;
            if resolved_types
                .iter()
                .any(|resolved| std::ptr::eq(*resolved, defined_type))
            {
                return Err(unresolvable(segment, node));
            }
            resolved_types.push(defined_type);
            self.program = program.identifier();
            node = &defined_type.r#type;
        }
        Ok(node)
    }

    fn resolve_account_link(
        &mut self,
        link: &AccountLinkNode,
    ) -> Result<TypePathNode<'t>, PathError<'t>> {
        if self.account_link_resolution == AccountLinkResolution::Forbid {
            return Err(PathError::ForbiddenAccountLink);
        }
        let program = self.linked_program(link.program.as_ref())?;
        let account = program
            .account(&link.identifier)
            .ok_or(PathError::DanglingLink)?;
        self.program = program.identifier();
        Ok(TypePathNode::Type(&account.data))
    }

    /// The program that a link in the current program points into.
    fn linked_program(
        &self,
        link_program: Option<&ProgramLinkNode>,
    ) -> Result<&'l ProgramScope<'t>, PathError<'t>> {
        self.linkables
            .linked_program(link_program, Some(self.program))
            .ok_or(PathError::DanglingLink)
    }
}

impl<'a> TypePathNode<'a> {
    /// The type of a struct field, or the type itself.
    fn type_node(self) -> &'a TypeNode {
        match self {
            TypePathNode::Field(field) => field.r#type.as_ref(),
            TypePathNode::Type(node) => node,
        }
    }
}

fn unresolvable<'a>(segment: PathKey<'_>, reached: &'a TypeNode) -> PathError<'a> {
    PathError::Unresolvable {
        segment: segment.to_string(),
        reached,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{id, path};
    use crate::{
        AccountLinkNode, AccountNode, ArrayTypeNode, DefinedTypeLinkNode, EnumTypeNode,
        EnumVariantTypeNode, IntegerTypeNode, MapTypeNode, OptionTypeNode, ProgramLinkNode,
        ProgramNode, RootNode, SetTypeNode, StringTypeNode, StructTypeNode, TupleTypeNode, U16,
        U64, U8,
    };
    use pretty_assertions::assert_eq;
    use AccountLinkResolution::{Allow, Forbid};

    fn field(identifier: &str, r#type: impl Into<TypeNode>) -> StructFieldTypeNode {
        StructFieldTypeNode::new(id(identifier), r#type)
    }

    fn public_key_with_account_link(link: AccountLinkNode) -> PublicKeyTypeNode {
        PublicKeyTypeNode {
            account_link: Some(link),
            ..PublicKeyTypeNode::new()
        }
    }

    fn root() -> RootNode {
        RootNode::new(ProgramNode::new(id("token"), "1111"))
    }

    fn resolve<'a>(
        root: &'a RootNode,
        data: &'a TypeNode,
        value: &str,
        account_link_resolution: AccountLinkResolution,
    ) -> Result<TypePathNode<'a>, PathError<'a>> {
        let linkables = Linkables::new(root);
        let program = &root.program.identifier;
        resolve_type_path(
            &linkables,
            program,
            data,
            &path(value),
            account_link_resolution,
        )
    }

    fn unresolvable_error<'a>(segment: &str, reached: &'a TypeNode) -> PathError<'a> {
        PathError::Unresolvable {
            segment: segment.to_string(),
            reached,
        }
    }

    #[test]
    fn struct_fields() {
        let amount = field("amount", IntegerTypeNode::le(U64));
        let config = field("config", StructTypeNode::new(vec![amount.clone()]));
        let data = StructTypeNode::new(vec![config.clone()]).into();
        let root = root();
        assert_eq!(
            resolve(&root, &data, "config", Forbid),
            Ok(TypePathNode::Field(&config))
        );
        assert_eq!(
            resolve(&root, &data, "config.amount", Forbid),
            Ok(TypePathNode::Field(&amount))
        );
    }

    #[test]
    fn field_identifier_matches_exactly() {
        let data =
            StructTypeNode::new(vec![field("token_amount", IntegerTypeNode::le(U64))]).into();
        let root = root();
        for segment in ["Token_Amount", "tokenAmount"] {
            assert_eq!(
                resolve(&root, &data, segment, Forbid),
                Err(unresolvable_error(segment, &data))
            );
        }
    }

    #[test]
    fn a_struct_field_named_data() {
        let data_field = field(
            "data",
            public_key_with_account_link(AccountLinkNode::new(id("mint"))),
        );
        let data = StructTypeNode::new(vec![data_field.clone()]).into();
        let root = root();
        assert_eq!(
            resolve(&root, &data, "data", Forbid),
            Ok(TypePathNode::Field(&data_field))
        );
    }

    #[test]
    fn tuple_array_and_set_items() {
        let tuple = TupleTypeNode::new(vec![
            IntegerTypeNode::le(U8).into(),
            PublicKeyTypeNode::new().into(),
        ])
        .into();
        let array = ArrayTypeNode::fixed(IntegerTypeNode::le(U16), 2).into();
        let set = SetTypeNode::fixed(StringTypeNode::utf8(), 2).into();
        let root = root();
        assert_eq!(
            resolve(&root, &tuple, "[1]", Forbid),
            Ok(TypePathNode::Type(&PublicKeyTypeNode::new().into()))
        );
        assert_eq!(
            resolve(&root, &array, "[5]", Forbid),
            Ok(TypePathNode::Type(&IntegerTypeNode::le(U16).into()))
        );
        assert_eq!(
            resolve(&root, &set, "[0]", Forbid),
            Ok(TypePathNode::Type(&StringTypeNode::utf8().into()))
        );
    }

    #[test]
    fn links_to_other_programs() {
        let amount = field("amount", IntegerTypeNode::le(U16));
        let fees = field(
            "fees",
            ArrayTypeNode::fixed(DefinedTypeLinkNode::new(id("fee")), 2),
        );
        let program_b = ProgramNode::new(id("programB"), "2222")
            .add_defined_type(DefinedTypeNode::new(
                id("config"),
                StructTypeNode::new(vec![fees]),
            ))
            .add_defined_type(DefinedTypeNode::new(
                id("fee"),
                StructTypeNode::new(vec![amount.clone()]),
            ));
        let config_link = DefinedTypeLinkNode::new_from_program(
            id("config"),
            ProgramLinkNode::new(id("programB")),
        );
        let data = StructTypeNode::new(vec![field("config", config_link)]).into();
        let root = RootNode::new(ProgramNode::new(id("programA"), "1111")).add_program(program_b);
        assert_eq!(
            resolve(&root, &data, "config.fees[0]", Forbid),
            Ok(TypePathNode::Type(
                &DefinedTypeLinkNode::new(id("fee")).into()
            ))
        );
        assert_eq!(
            resolve(&root, &data, "config.fees[0].amount", Forbid),
            Ok(TypePathNode::Field(&amount))
        );
    }

    #[test]
    fn last_node_as_addressed() {
        let config_link = DefinedTypeLinkNode::new(id("config"));
        let config = field("config", config_link.clone());
        let items = field(
            "items",
            TupleTypeNode::new(vec![config_link.clone().into()]),
        );
        let data = StructTypeNode::new(vec![items, config.clone()]).into();
        let root = RootNode::new(ProgramNode::new(id("token"), "1111").add_defined_type(
            DefinedTypeNode::new(id("config"), StructTypeNode::new(vec![])),
        ));
        assert_eq!(
            resolve(&root, &data, "config", Forbid),
            Ok(TypePathNode::Field(&config))
        );
        assert_eq!(
            resolve(&root, &data, "items[0]", Forbid),
            Ok(TypePathNode::Type(&config_link.into()))
        );
    }

    #[test]
    fn segments_that_do_not_apply() {
        let amount_type = IntegerTypeNode::le(U64).into();
        let data = StructTypeNode::new(vec![field("amount", IntegerTypeNode::le(U64))]).into();
        let tuple = TupleTypeNode::new(vec![IntegerTypeNode::le(U8).into()]).into();
        let root = root();
        assert_eq!(
            resolve(&root, &data, "fee", Forbid),
            Err(unresolvable_error("fee", &data))
        );
        assert_eq!(
            resolve(&root, &data, "amount[0]", Forbid),
            Err(unresolvable_error("[0]", &amount_type))
        );
        assert_eq!(
            resolve(&root, &data, "amount.value", Forbid),
            Err(unresolvable_error("value", &amount_type))
        );
        assert_eq!(
            resolve(&root, &data, "[0]", Forbid),
            Err(unresolvable_error("[0]", &data))
        );
        assert_eq!(
            resolve(&root, &tuple, "first", Forbid),
            Err(unresolvable_error("first", &tuple))
        );
        assert_eq!(
            resolve(&root, &tuple, "[1]", Forbid),
            Err(unresolvable_error("[1]", &tuple))
        );
    }

    #[test]
    fn option_map_and_enum_types_are_not_entered() {
        let config = StructTypeNode::new(vec![field("fee", IntegerTypeNode::le(U64))]);
        let option = OptionTypeNode::new(config.clone()).into();
        let map = MapTypeNode::fixed(StringTypeNode::utf8(), config.clone(), 2).into();
        let enum_type = EnumTypeNode::new(vec![
            EnumVariantTypeNode::new(id("empty")),
            EnumVariantTypeNode::with_data(id("config"), config),
        ])
        .into();
        let root = root();
        assert_eq!(
            resolve(&root, &option, "fee", Allow),
            Err(unresolvable_error("fee", &option))
        );
        assert_eq!(
            resolve(&root, &map, "[0]", Allow),
            Err(unresolvable_error("[0]", &map))
        );
        assert_eq!(
            resolve(&root, &enum_type, "config", Allow),
            Err(unresolvable_error("config", &enum_type))
        );
    }

    #[test]
    fn dangling_defined_type_link() {
        let data = StructTypeNode::new(vec![field(
            "config",
            DefinedTypeLinkNode::new(id("missing")),
        )])
        .into();
        let root = root();
        assert_eq!(
            resolve(&root, &data, "config.fee", Forbid),
            Err(PathError::DanglingLink)
        );
    }

    #[test]
    fn alias_cycle() {
        let root = RootNode::new(
            ProgramNode::new(id("token"), "1111")
                .add_defined_type(DefinedTypeNode::new(
                    id("a"),
                    DefinedTypeLinkNode::new(id("b")),
                ))
                .add_defined_type(DefinedTypeNode::new(
                    id("b"),
                    DefinedTypeLinkNode::new(id("a")),
                )),
        );
        let data = DefinedTypeLinkNode::new(id("a")).into();
        assert_eq!(
            resolve(&root, &data, "x", Forbid),
            Err(unresolvable_error("x", &data))
        );
    }

    fn token_root() -> RootNode {
        let mint_field = field(
            "mint",
            public_key_with_account_link(AccountLinkNode::new(id("mint"))),
        );
        RootNode::new(
            ProgramNode::new(id("token"), "1111")
                .add_account(AccountNode::new(
                    id("token"),
                    StructTypeNode::new(vec![mint_field]),
                ))
                .add_account(AccountNode::new(
                    id("mint"),
                    StructTypeNode::new(vec![field("decimals", IntegerTypeNode::le(U8))]),
                )),
        )
    }

    #[test]
    fn account_link_with_a_data_segment() {
        let root = token_root();
        let token_data = &root.program.accounts[0].data;
        let mint_data = &root.program.accounts[1].data;
        assert_eq!(
            resolve(&root, token_data, "mint.data.decimals", Allow),
            Ok(TypePathNode::Field(&field(
                "decimals",
                IntegerTypeNode::le(U8)
            )))
        );
        assert_eq!(
            resolve(&root, token_data, "mint.data", Allow),
            Ok(TypePathNode::Type(mint_data))
        );
        assert_eq!(
            resolve(&root, token_data, "mint.data", Forbid),
            Err(PathError::ForbiddenAccountLink)
        );
    }

    #[test]
    fn account_link_with_other_segments() {
        let root = token_root();
        let token_data = &root.program.accounts[0].data;
        let mint_key = public_key_with_account_link(AccountLinkNode::new(id("mint"))).into();
        for (value, segment) in [("mint.decimals", "decimals"), ("mint[0]", "[0]")] {
            assert_eq!(
                resolve(&root, token_data, value, Allow),
                Err(unresolvable_error(segment, &mint_key)),
                "{value}"
            );
        }
        assert_eq!(
            resolve(&root, token_data, "mint", Allow),
            Ok(TypePathNode::Field(&field("mint", mint_key.clone())))
        );
    }

    #[test]
    fn data_segment_on_a_public_key_without_account_link() {
        let key = PublicKeyTypeNode::new().into();
        let data = StructTypeNode::new(vec![field("owner", PublicKeyTypeNode::new())]).into();
        let root = root();
        for account_link_resolution in [Allow, Forbid] {
            assert_eq!(
                resolve(&root, &data, "owner.data", account_link_resolution),
                Err(unresolvable_error("data", &key))
            );
        }
    }

    #[test]
    fn account_links_back_to_the_same_account() {
        let node_data = StructTypeNode::new(vec![
            field("value", IntegerTypeNode::le(U8)),
            field(
                "next",
                public_key_with_account_link(AccountLinkNode::new(id("node"))),
            ),
        ]);
        let root = RootNode::new(
            ProgramNode::new(id("list"), "2222")
                .add_account(AccountNode::new(id("node"), node_data)),
        );
        let data = &root.program.accounts[0].data;
        assert_eq!(
            resolve(&root, data, "next.data.next.data.value", Allow),
            Ok(TypePathNode::Field(&field(
                "value",
                IntegerTypeNode::le(U8)
            )))
        );
    }

    #[test]
    fn dangling_account_link() {
        let data = StructTypeNode::new(vec![field(
            "missing",
            public_key_with_account_link(AccountLinkNode::new(id("missing"))),
        )])
        .into();
        let root = root();
        assert_eq!(
            resolve(&root, &data, "missing.data", Allow),
            Err(PathError::DanglingLink)
        );
    }

    #[test]
    fn account_link_to_another_program() {
        let mint_link =
            AccountLinkNode::new_from_program(id("mint"), ProgramLinkNode::new(id("token")));
        let data =
            StructTypeNode::new(vec![field("mint", public_key_with_account_link(mint_link))])
                .into();
        let decimals = field("decimals", IntegerTypeNode::le(U8));
        let mint_data = StructTypeNode::new(vec![field(
            "config",
            DefinedTypeLinkNode::new(id("mint_config")),
        )]);
        let token = ProgramNode::new(id("token"), "2222")
            .add_account(AccountNode::new(id("mint"), mint_data))
            .add_defined_type(DefinedTypeNode::new(
                id("mint_config"),
                StructTypeNode::new(vec![decimals.clone()]),
            ));
        let root = RootNode::new(ProgramNode::new(id("app"), "1111")).add_program(token);
        assert_eq!(
            resolve(&root, &data, "mint.data.config.decimals", Allow),
            Ok(TypePathNode::Field(&decimals))
        );
    }
}
