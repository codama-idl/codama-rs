use std::path::Path;

use cargo_toml::Manifest;

use crate::internals::{ParsingError, ParsingResult};
use crate::modules::{UnparsedCrate, UnparsedRoot};
use crate::nodes::NumberTypeNode;
use crate::{attributes::Attribute, nodes::TypeNode};

pub struct RootKorok<'a> {
    pub crates: Vec<CrateKorok<'a>>,
}

impl<'a> RootKorok<'a> {
    pub fn parse(unparsed_root: &'a UnparsedRoot) -> ParsingResult<Self> {
        Ok(Self {
            crates: unparsed_root
                .crates
                .iter()
                .map(CrateKorok::parse)
                .collect::<ParsingResult<_>>()?,
        })
    }
}

pub struct CrateKorok<'a> {
    pub file: &'a syn::File,
    pub path: &'a Path,
    pub items: Vec<ItemKorok<'a>>,
    pub manifest: &'a Manifest,
}

impl<'a> CrateKorok<'a> {
    pub fn parse(unparsed_crate: &'a UnparsedCrate) -> ParsingResult<Self> {
        Ok(Self {
            file: &unparsed_crate.file,
            path: &unparsed_crate.path,
            items: Vec::new(),
            manifest: &unparsed_crate.manifest,
        })
    }
}

pub enum ItemKorok<'a> {
    FileModule(FileModuleKorok<'a>),
    Module(ModuleKorok<'a>),
    Struct(StructKorok<'a>),
    Enum(EnumKorok<'a>),
    Unsupported(UnsupportedItemKorok<'a>),
}

impl<'a> ItemKorok<'a> {
    pub fn parse(item: &'a syn::Item) -> ParsingResult<Self> {
        match item {
            // syn::Item::Mod(item) if item.content.is_some() => {}
            // syn::Item::Mod(item) if item.content.is_none() => {}
            syn::Item::Struct(item) => Ok(ItemKorok::Struct(StructKorok::parse(item)?)),
            syn::Item::Enum(item) => Ok(ItemKorok::Enum(EnumKorok::parse(item)?)),
            _ => Ok(ItemKorok::Unsupported(UnsupportedItemKorok { ast: item })),
        }
    }

    pub fn parse_all(items: &'a Vec<syn::Item>) -> ParsingResult<Vec<Self>> {
        items.iter().map(|item| Self::parse(item)).collect()
    }
}

pub struct FileModuleKorok<'a> {
    pub file: &'a syn::File,
    pub ast: &'a syn::ItemMod,
    pub path: &'a Path,
    pub items: Vec<ItemKorok<'a>>,
}

impl<'a> FileModuleKorok<'a> {
    pub fn parse(_ast: &'a syn::ItemMod) -> ParsingResult<Self> {
        unimplemented!()
    }
}

pub struct ModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub path: &'a Path,
    pub items: Vec<ItemKorok<'a>>,
}

pub struct StructKorok<'a> {
    pub ast: &'a syn::ItemStruct,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: Vec<FieldKorok<'a>>,
}

impl<'a> StructKorok<'a> {
    fn parse(ast: &'a syn::ItemStruct) -> ParsingResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldKorok::parse_all(&ast.fields)?,
        })
    }
}

pub struct FieldKorok<'a> {
    pub ast: &'a syn::Field,
    pub base_type: TypeNode,
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> FieldKorok<'a> {
    pub fn parse_all(fields: &'a syn::Fields) -> ParsingResult<Vec<Self>> {
        match fields {
            syn::Fields::Named(f) => f.named.iter().map(Self::try_from).collect(),
            syn::Fields::Unnamed(f) => f.unnamed.iter().map(Self::try_from).collect(),
            syn::Fields::Unit => Ok(vec![]),
        }
    }
}

impl<'a> TryFrom<&'a syn::Field> for FieldKorok<'a> {
    type Error = ParsingError;

    fn try_from(ast: &'a syn::Field) -> ParsingResult<Self> {
        let attributes = Attribute::parse_all(&ast.attrs)?;
        // TODO: implement.
        let base_type = TypeNode::Number(NumberTypeNode {
            format: "u16".to_string(),
        });
        Ok(Self {
            ast,
            base_type,
            attributes,
        })
    }
}

pub struct EnumKorok<'a> {
    pub ast: &'a syn::ItemEnum,
    pub attributes: Vec<Attribute<'a>>,
    pub variants: Vec<EnumVariantKorok<'a>>,
}

impl<'a> EnumKorok<'a> {
    fn parse(ast: &'a syn::ItemEnum) -> ParsingResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            variants: EnumVariantKorok::parse_all(&ast.variants)?,
        })
    }
}

pub struct EnumVariantKorok<'a> {
    pub ast: &'a syn::Variant,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: Vec<FieldKorok<'a>>,
}

impl<'a> EnumVariantKorok<'a> {
    fn parse(ast: &'a syn::Variant) -> ParsingResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldKorok::parse_all(&ast.fields)?,
        })
    }

    pub fn parse_all(
        variants: &'a syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> ParsingResult<Vec<Self>> {
        variants.iter().map(Self::parse).collect()
    }
}

pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
}
