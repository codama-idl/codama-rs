use codama_attributes::{
    Attribute, AttributeContext, Attributes, CodamaDirective, Resolvable, SeedDirectiveType,
};
use codama_errors::CodamaResult;
use codama_nodes::{BytesEncoding, Endian, Number, NumberFormat, TypeNode, ValueNode};
use codama_syn_helpers::{snake_case_ident, string_to_ident, to_snake_case};
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::HashMap;

/// The result of parsing all `#[codama(seed(...))]` attributes on a struct.
struct ParsedSeeds {
    /// Ordered seed expressions (constants and variable references) that map
    /// directly to the elements of the generated `seeds()` array.
    entries: Vec<ParsedSeedEntry>,
    /// Deduplicated variable seed parameters that become function arguments
    /// on the generated helper methods.
    parameters: Vec<SeedParameter>,
}

/// A single seed entry producing a `&[u8]` expression in the generated code.
struct ParsedSeedEntry {
    raw_expr: TokenStream,
}

/// A variable seed that becomes a function parameter on generated methods.
#[derive(Debug)]
struct SeedParameter {
    ident: syn::Ident,
}

/// Entry point for the `#[derive(CodamaPdaHelpers)]` macro. Parses seed
/// attributes from the struct and generates an `impl` block with PDA helper methods.
pub fn codama_pda_helpers_derive_impl(input: TokenStream) -> CodamaResult<TokenStream> {
    let item = syn::parse2(input)?;
    let item_struct = match &item {
        syn::Item::Struct(item_struct) => item_struct,
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "CodamaPdaHelpers currently supports structs only",
            )
            .into())
        }
    };

    if !item_struct.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_struct.generics,
            "CodamaPdaHelpers does not support generic structs",
        )
        .into());
    }

    // Extract only the #[codama(seed(...))] attributes from the struct.
    let attributes = Attributes::parse(&item_struct.attrs, AttributeContext::Item(&item))?;
    let seed_attributes = attributes
        .iter()
        .filter_map(|attribute| match attribute {
            Attribute::Codama(codama_attribute)
                if matches!(
                    codama_attribute.directive.as_ref(),
                    CodamaDirective::Seed(_)
                ) =>
            {
                Some(codama_attribute)
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    if seed_attributes.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_struct.ident,
            "CodamaPdaHelpers requires at least one #[codama(seed(...))] attribute",
        )
        .into());
    }

    // Convert seed attributes into expressions and parameters
    let parsed_seeds = parse_seed_sequence(&seed_attributes)?;

    // generate the impl block with all helper methods.
    Ok(generate_impl(&item_struct.ident, &parsed_seeds))
}

/// Walks the seed attributes in declaration order and builds two lists:
/// - `entries`: one expression per seed, in order, for the generated `seeds()` array
/// - `parameters`: deduplicated variable seeds that become function arguments
fn parse_seed_sequence(
    seed_attributes: &[&codama_attributes::CodamaAttribute<'_>],
) -> CodamaResult<ParsedSeeds> {
    let mut entries = Vec::with_capacity(seed_attributes.len());
    let mut parameters = Vec::new();

    // Maps seed name -> generated ident, for deduplicating repeated seeds
    let mut parameters_by_name = HashMap::<String, syn::Ident>::new();

    // Maps generated ident string -> original seed name, for collision detection.
    // Pre-seeded with "bump" since the generated methods always use that name.
    let mut generated_names = HashMap::<String, String>::new();
    generated_names.insert("bump".to_string(), "<generated bump parameter>".to_string());

    for seed_attribute in seed_attributes {
        let CodamaDirective::Seed(seed_directive) = seed_attribute.directive.as_ref() else {
            unreachable!("parse_seed_sequence is only called with seed directives");
        };

        let entry = match &seed_directive.seed {
            // Constant seeds are encoded as byte literals at compile time.
            SeedDirectiveType::Constant { r#type, value } => {
                parse_constant_seed(seed_attribute, r#type, value)?
            }
            // Variable and linked seeds become runtime function parameters.
            SeedDirectiveType::Variable { name, .. } | SeedDirectiveType::Linked(name) => {
                // Reuse an existing parameter if this seed name was already seen,
                // otherwise create a new one and check for identifier collisions.
                let ident = match parameters_by_name.get(name) {
                    Some(ident) => ident.clone(),
                    None => {
                        let ident = seed_name_to_ident(name, parameters.len());
                        let ident_key = ident.to_string();
                        if let Some(existing_name) = generated_names.get(&ident_key) {
                            return Err(syn::Error::new_spanned(
                                seed_attribute.ast,
                                format!(
                                    "CodamaPdaHelpers generated a duplicate parameter name `{ident_key}` from seed `{name}`; it conflicts with `{existing_name}`"
                                ),
                            )
                            .into());
                        }

                        generated_names.insert(ident_key, name.clone());
                        parameters_by_name.insert(name.clone(), ident.clone());
                        parameters.push(SeedParameter {
                            ident: ident.clone(),
                        });
                        ident
                    }
                };

                ParsedSeedEntry {
                    raw_expr: quote! { #ident.as_ref() },
                }
            }
        };

        entries.push(entry);
    }

    Ok(ParsedSeeds {
        entries,
        parameters,
    })
}

/// Converts a constant seed's type and value into a byte-string literal
fn parse_constant_seed(
    seed_attribute: &codama_attributes::CodamaAttribute<'_>,
    r#type: &Resolvable<TypeNode>,
    value: &Resolvable<ValueNode>,
) -> CodamaResult<ParsedSeedEntry> {
    // Ensure both type and value are fully resolved (not plugin directives)
    let type_node = r#type.try_resolved().map_err(|error| {
        syn::Error::new_spanned(
            seed_attribute.ast,
            format!("CodamaPdaHelpers requires a resolved constant seed type: {error}"),
        )
    })?;
    let value_node = value.try_resolved().map_err(|error| {
        syn::Error::new_spanned(
            seed_attribute.ast,
            format!("CodamaPdaHelpers requires a resolved constant seed value: {error}"),
        )
    })?;

    match (type_node, value_node) {
        // UTF-8 string seeds: "vault" -> b"vault"
        (TypeNode::String(string_type), ValueNode::String(string_value))
            if string_type.encoding == BytesEncoding::Utf8 =>
        {
            let literal = Literal::byte_string(string_value.string.as_bytes());
            Ok(ParsedSeedEntry {
                raw_expr: quote! { #literal },
            })
        }
        // Integer seeds: encode the value as bytes
        (TypeNode::Number(number_type), ValueNode::Number(number_value)) => {
            let bytes = encode_number_seed_bytes(number_type.format, number_type.endian, number_value.number)
                .map_err(|message| syn::Error::new_spanned(seed_attribute.ast, message))?;
            let literal = Literal::byte_string(&bytes);
            Ok(ParsedSeedEntry {
                raw_expr: quote! { #literal },
            })
        }
        _ => Err(syn::Error::new_spanned(
            seed_attribute.ast,
            "CodamaPdaHelpers currently supports constant seeds only for string(utf8) and integer number(...) types",
        )
        .into()),
    }
}

/// Encodes a numeric constant seed value into its byte representation.
fn encode_number_seed_bytes(
    format: NumberFormat,
    endian: Endian,
    number: Number,
) -> Result<Vec<u8>, String> {
    macro_rules! encode_unsigned {
        ($ty:ty, $label:expr) => {{
            let n = <$ty>::try_from(as_unsigned(number, $label)?).map_err(|_| {
                format!(
                    "CodamaPdaHelpers constant number({}) is out of range",
                    $label
                )
            })?;
            Ok(match endian {
                Endian::Little => n.to_le_bytes(),
                Endian::Big => n.to_be_bytes(),
            }
            .to_vec())
        }};
    }

    macro_rules! encode_signed {
        ($ty:ty, $label:expr) => {{
            let n = <$ty>::try_from(as_signed(number, $label)?).map_err(|_| {
                format!(
                    "CodamaPdaHelpers constant number({}) is out of range",
                    $label
                )
            })?;
            Ok(match endian {
                Endian::Little => n.to_le_bytes(),
                Endian::Big => n.to_be_bytes(),
            }
            .to_vec())
        }};
    }

    match format {
        NumberFormat::U8 => encode_unsigned!(u8, "u8"),
        NumberFormat::U16 => encode_unsigned!(u16, "u16"),
        NumberFormat::U32 => encode_unsigned!(u32, "u32"),
        NumberFormat::U64 => encode_unsigned!(u64, "u64"),
        NumberFormat::U128 => encode_unsigned!(u128, "u128"),
        NumberFormat::I8 => encode_signed!(i8, "i8"),
        NumberFormat::I16 => encode_signed!(i16, "i16"),
        NumberFormat::I32 => encode_signed!(i32, "i32"),
        NumberFormat::I64 => encode_signed!(i64, "i64"),
        NumberFormat::I128 => encode_signed!(i128, "i128"),
        NumberFormat::ShortU16 => Err(
            "CodamaPdaHelpers does not yet support constant number(short_u16) seeds".to_string(),
        ),
        NumberFormat::F32 | NumberFormat::F64 => {
            Err("CodamaPdaHelpers does not support floating-point constant seeds".to_string())
        }
    }
}

/// Extracts the value from a `Number` as `u64`.
fn as_unsigned(number: Number, label: &str) -> Result<u64, String> {
    match number {
        Number::UnsignedInteger(value) => Ok(value),
        Number::SignedInteger(_) => Err(format!(
            "CodamaPdaHelpers constant number({label}) must use an unsigned integer literal"
        )),
        Number::Float(_) => Err(format!(
            "CodamaPdaHelpers constant number({label}) must not use a floating-point literal"
        )),
    }
}

/// Extracts the value from a `Number` as `i64`.
fn as_signed(number: Number, label: &str) -> Result<i64, String> {
    match number {
        Number::SignedInteger(value) => Ok(value),
        Number::UnsignedInteger(value) => i64::try_from(value)
            .map_err(|_| format!("CodamaPdaHelpers constant number({label}) is out of range")),
        Number::Float(_) => Err(format!(
            "CodamaPdaHelpers constant number({label}) must not use a floating-point literal"
        )),
    }
}

/// Generates the `impl` block containing all PDA helper methods.
///
/// Each method has two variants depending on whether there are variable seed
/// parameters:
///
/// ```ignore
/// // Constant-only seeds:
/// fn seeds() -> [&'static [u8]; 1] { [b"vault"] }
///
/// // With variable seeds:
/// fn seeds<'a>(authority: &'a (impl AsRef<[u8]> + 'a)) -> [&'a [u8]; 2] {
///     [b"vault", authority.as_ref()]
/// }
/// ```
///
/// Generated methods:
/// - `seeds` / `seeds_with_bump`
/// - `signer_seeds`
///
/// (wrapping `Address` methods)
/// - `derive_address`
/// - `create_program_address`
/// - `find_program_address`
/// - `try_find_program_address`
/// - `derive_program_address`
fn generate_impl(ident: &syn::Ident, parsed_seeds: &ParsedSeeds) -> TokenStream {
    // Build reusable token fragments from the parsed seeds
    let param_defs = parsed_seeds
        .parameters
        .iter()
        .map(|parameter| {
            let ident = &parameter.ident;
            quote! { #ident: &'a (impl AsRef<[u8]> + 'a) }
        })
        .collect::<Vec<_>>();
    let seed_exprs = parsed_seeds
        .entries
        .iter()
        .map(|entry| entry.raw_expr.clone())
        .collect::<Vec<_>>();
    let param_idents = parsed_seeds
        .parameters
        .iter()
        .map(|parameter| parameter.ident.clone())
        .collect::<Vec<_>>();
    let seeds_len = parsed_seeds.entries.len();
    let seeds_with_bump_len = seeds_len + 1;

    // seeds() - returns the seed byte slices as a fixed-size array, without the bump.
    // This is the base array used by all other helpers.
    let seeds = if parsed_seeds.parameters.is_empty() {
        quote! {
            pub fn seeds() -> [&'static [u8]; #seeds_len] {
                [#(#seed_exprs),*]
            }
        }
    } else {
        quote! {
            #[allow(clippy::needless_lifetimes)]
            pub fn seeds<'a>(#(#param_defs),*) -> [&'a [u8]; #seeds_len] {
                [#(#seed_exprs),*]
            }
        }
    };

    // seeds_with_bump() - like seeds() but appends the bump byte slice at the end.
    // Used by create_program_address and signer_seeds.
    let seeds_with_bump = quote! {
        #[allow(clippy::needless_lifetimes)]
        pub fn seeds_with_bump<'a>(#(#param_defs,)* bump: &'a [u8]) -> [&'a [u8]; #seeds_with_bump_len] {
            [#(#seed_exprs,)* bump]
        }
    };

    // signer_seeds() - like seeds_with_bump(), but converts each element to a
    // generic type T (e.g. pinocchio::Seed) for use with CPI signing
    let signer_seeds = quote! {
        #[allow(clippy::needless_lifetimes)]
        pub fn signer_seeds<'a, T>(#(#param_defs,)* bump: &'a [u8]) -> [T; #seeds_with_bump_len]
        where
            T: From<&'a [u8]>,
        {
            Self::seeds_with_bump(#(#param_idents,)* bump).map(T::from)
        }
    };

    // derive_address() - computes the PDA address via SHA-256 hashing without
    // curve validation. Wraps Address::derive_address.
    let derive_address = quote! {
        #[allow(clippy::needless_lifetimes)]
        pub fn derive_address<'a>(
            #(#param_defs,)*
            bump: Option<u8>,
            program_id: &::solana_address::Address,
        ) -> ::solana_address::Address {
            ::solana_address::Address::derive_address(
                &Self::seeds(#(#param_idents),*),
                bump,
                program_id,
            )
        }
    };

    // create_program_address() - hashes seeds + bump and validates the result is
    // off the ed25519 curve (a valid PDA). Wraps Address::create_program_address.
    let create_program_address = quote! {
        #[allow(clippy::needless_lifetimes)]
        pub fn create_program_address<'a>(
            #(#param_defs,)*
            bump: u8,
            program_id: &::solana_address::Address,
        ) -> Result<::solana_address::Address, ::solana_address::error::AddressError> {
            let bump_seed = [bump];
            ::solana_address::Address::create_program_address(
                &Self::seeds_with_bump(#(#param_idents,)* &bump_seed),
                program_id,
            )
        }
    };

    // find_program_address() - searches for a valid PDA by trying bumps from 255
    // down to 0 via create_program_address. Wraps Address::find_program_address.
    let find_program_address = quote! {
        #[allow(clippy::needless_lifetimes)]
        pub fn find_program_address<'a>(
            #(#param_defs,)*
            program_id: &::solana_address::Address,
        ) -> (::solana_address::Address, u8) {
            ::solana_address::Address::find_program_address(
                &Self::seeds(#(#param_idents),*),
                program_id,
            )
        }
    };

    // try_find_program_address() - same bump search as find_program_address() but
    // returns None instead of panicking. Wraps Address::try_find_program_address.
    let try_find_program_address = quote! {
        #[allow(clippy::needless_lifetimes)]
        pub fn try_find_program_address<'a>(
            #(#param_defs,)*
            program_id: &::solana_address::Address,
        ) -> Option<(::solana_address::Address, u8)> {
            ::solana_address::Address::try_find_program_address(
                &Self::seeds(#(#param_idents),*),
                program_id,
            )
        }
    };

    // derive_program_address() - like try_find_program_address() but uses
    // derive_address + is_on_curve internally. Wraps Address::derive_program_address.
    let derive_program_address = quote! {
        #[allow(clippy::needless_lifetimes)]
        pub fn derive_program_address<'a>(
            #(#param_defs,)*
            program_id: &::solana_address::Address,
        ) -> Option<(::solana_address::Address, u8)> {
            ::solana_address::Address::derive_program_address(
                &Self::seeds(#(#param_idents),*),
                program_id,
            )
        }
    };

    quote! {
        impl #ident {
            #seeds

            #seeds_with_bump

            #signer_seeds

            #derive_address

            #create_program_address

            #find_program_address

            #try_find_program_address

            #derive_program_address
        }
    }
}

/// Converts a seed name (e.g. "tokenProgram", "order-id") into a valid
/// snake_case Rust identifier. Names that produce an empty result after
/// conversion (e.g. "!!!") fall back to `seed_{index}`.
fn seed_name_to_ident(name: &str, index: usize) -> syn::Ident {
    let snake = to_snake_case(name);
    if snake.is_empty() {
        return string_to_ident(&format!("seed_{index}"));
    }
    snake_case_ident(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_name_to_ident_uses_snake_case() {
        assert_eq!(
            seed_name_to_ident("tokenProgram", 0).to_string(),
            "token_program"
        );
        assert_eq!(seed_name_to_ident("order-id", 0).to_string(), "order_id");
        assert_eq!(seed_name_to_ident("type", 0).to_string(), "r#type");
    }

    #[test]
    fn seed_name_to_ident_falls_back_when_empty() {
        assert_eq!(seed_name_to_ident("!!!", 3).to_string(), "seed_3");
    }
}
