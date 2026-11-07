use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyTypeOverridesVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetPdasVisitor,
};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    ConstantPdaSeedNode, IntegerFormat::U8, IntegerTypeNode, PdaNode, ProgramNode,
    PublicKeyTypeNode, StringTypeNode, StringValueNode, VariablePdaSeedNode,
};

#[test]
fn it_defines_pdas_from_structs() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(identifier = "authority", type = public_key))]
        #[codama(seed(identifier = "identifier", type = integer(u8)))]
        struct Counter;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", IntegerTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_defines_pdas_from_structs_with_linked_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(identifier = "authority"))]
        #[codama(seed(identifier = "identifier"))]
        struct Counter {
            identifier: u8,
            authority: Pubkey,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", IntegerTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_defines_pdas_from_structs_with_linked_fields_and_custom_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(identifier = "authority"))]
        #[codama(seed(identifier = "identifier"))]
        struct Counter {
            identifier: Overridden,
            authority: Overridden,
        }
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = Some(IntegerTypeNode::le(U8).into());
    korok.fields[1].node = Some(PublicKeyTypeNode::new().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", IntegerTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_defines_pdas_from_structs_with_linked_fields_and_type_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(identifier = "authority"))]
        #[codama(seed(identifier = "identifier"))]
        struct Counter {
            #[codama(type = integer(u8))]
            identifier: Overridden,
            #[codama(type = public_key)]
            authority: Overridden,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", IntegerTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_fails_on_linked_fields_without_a_type_node() -> CodamaResult<()> {
    for item in [
        syn::parse_quote! {
            #[derive(CodamaPda)]
            #[codama(seed(type = string(utf8), value = "vault"))]
            #[codama(seed(identifier = "authority"))]
            struct Vault {
                authority: [u8; SEED_LEN],
            }
        },
        syn::parse_quote! {
            #[derive(CodamaPda)]
            #[codama(seed(identifier = "authority"))]
            struct Vault {
                authority: Box<Pubkey>,
            }
        },
    ] {
        let mut korok = StructKorok::parse(&item)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        let error = korok.accept(&mut SetPdasVisitor::new()).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Seed `authority` links to a field whose type does not resolve, add `type = ...` to the seed"
        );
    }
    Ok(())
}

#[test]
fn it_fails_on_linked_seeds_without_a_matching_field() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "vault"))]
        #[codama(seed(identifier = "authority"))]
        struct Vault {
            authority: Pubkey,
        }
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields.clear();
    let error = korok.accept(&mut SetPdasVisitor::new()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Seed `authority` links to a field that does not exist, add `type = ...` to the seed"
    );
    Ok(())
}

#[test]
fn it_defines_pdas_from_enums() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(seed(type = string(utf8), value = "counter_pda"))]
        #[codama(seed(identifier = "authority", type = public_key))]
        #[codama(seed(identifier = "identifier", type = integer(u8)))]
        enum Counter { One, Two, Three }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            PdaNode::new(
                "counter",
                vec![
                    ConstantPdaSeedNode::new(
                        StringTypeNode::utf8(),
                        StringValueNode::new("counter_pda")
                    )
                    .into(),
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                    VariablePdaSeedNode::new("identifier", IntegerTypeNode::le(U8)).into(),
                ]
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_wraps_pda_in_program_node_with_program_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(program(identifier = "associatedToken", address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"))]
        #[codama(seed(identifier = "owner", type = public_key))]
        #[codama(seed(identifier = "tokenProgram", type = public_key))]
        #[codama(seed(identifier = "mint", type = public_key))]
        struct AssociatedToken;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                identifier: "associatedToken".into(),
                public_key: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".to_string(),
                pdas: vec![PdaNode::new(
                    "associated_token",
                    vec![
                        VariablePdaSeedNode::new("owner", PublicKeyTypeNode::new()).into(),
                        VariablePdaSeedNode::new("tokenProgram", PublicKeyTypeNode::new()).into(),
                        VariablePdaSeedNode::new("mint", PublicKeyTypeNode::new()).into(),
                    ]
                )],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_wraps_pda_in_program_node_from_enum_with_program_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaPda)]
        #[codama(program(identifier = "associatedToken", address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"))]
        #[codama(seed(identifier = "owner", type = public_key))]
        #[codama(seed(identifier = "mint", type = public_key))]
        enum AssociatedToken { V1, V2 }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetPdasVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                identifier: "associatedToken".into(),
                public_key: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".to_string(),
                pdas: vec![PdaNode::new(
                    "associated_token",
                    vec![
                        VariablePdaSeedNode::new("owner", PublicKeyTypeNode::new()).into(),
                        VariablePdaSeedNode::new("mint", PublicKeyTypeNode::new()).into(),
                    ]
                )],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
