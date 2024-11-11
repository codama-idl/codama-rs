use std::fs;
use std::path::Path;

use cargo_toml::Manifest;

use crate::internals::{ParsingError, ParsingResult};
use crate::nodes::NumberTypeNode;
use crate::{attributes::Attribute, nodes::TypeNode};

pub struct RootKorok<'a> {
    pub crates: Vec<CrateKorok<'a>>,
}

pub struct CrateKorok<'a> {
    pub file: syn::File,
    pub path: &'a Path,
    pub items: Vec<ItemKorok<'a>>,
    pub manifest: Manifest,
}

impl<'a> CrateKorok<'a> {
    pub fn parse(path: &'a Path) -> ParsingResult<Self> {
        let content = fs::read_to_string(path)?;
        let file = syn::parse_file(&content)?;
        let manifest = Manifest::from_path(path)?;
        let mut korok = Self {
            file,
            path,
            items: vec![],
            manifest,
        };
        korok.items = ItemKorok::parse_all(&korok.file.items)?;

        Ok(korok)
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
    pub fn parse_all(items: &'a Vec<syn::Item>) -> ParsingResult<Vec<Self>> {
        items.iter().map(Self::try_from).collect()
    }
}

impl<'a> TryFrom<&'a syn::Item> for ItemKorok<'a> {
    type Error = ParsingError;

    fn try_from(ast: &'a syn::Item) -> ParsingResult<Self> {
        // TODO: implement.
        match *ast {
            // syn::Item::Mod(item) if item.content.is_some() => {}
            // syn::Item::Mod(item) if item.content.is_none() => {}
            syn::Item::Struct(item) => Ok(ItemKorok::Struct(item.try_into()?)),
            syn::Item::Enum(item) => Ok(ItemKorok::Enum(item.try_into()?)),
            _ => Ok(ItemKorok::Unsupported(UnsupportedItemKorok { ast: ast })),
        }
    }
}

pub struct FileModuleKorok<'a> {
    pub file: syn::File,
    pub ast: &'a syn::ItemMod,
    pub path: &'a Path,
    pub items: Vec<ItemKorok<'a>>,
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

impl<'a> TryFrom<&'a syn::ItemStruct> for StructKorok<'a> {
    type Error = ParsingError;

    fn try_from(ast: &'a syn::ItemStruct) -> ParsingResult<Self> {
        let attributes = Attribute::parse_all(&ast.attrs)?;
        let fields = FieldKorok::parse_all(&ast.fields)?;
        Ok(Self {
            ast,
            attributes,
            fields,
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

impl<'a> TryFrom<&'a syn::ItemEnum> for EnumKorok<'a> {
    type Error = ParsingError;

    fn try_from(ast: &'a syn::ItemEnum) -> ParsingResult<Self> {
        let attributes = Attribute::parse_all(&ast.attrs)?;
        let variants = EnumVariantKorok::parse_all(&ast.variants)?;
        Ok(Self {
            ast,
            attributes,
            variants,
        })
    }
}

pub struct EnumVariantKorok<'a> {
    pub ast: &'a syn::Variant,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: Vec<FieldKorok<'a>>,
}

impl<'a> EnumVariantKorok<'a> {
    pub fn parse_all(
        variants: &'a syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> ParsingResult<Vec<Self>> {
        variants.iter().map(Self::try_from).collect()
    }
}

impl<'a> TryFrom<&'a syn::Variant> for EnumVariantKorok<'a> {
    type Error = ParsingError;

    fn try_from(ast: &'a syn::Variant) -> ParsingResult<Self> {
        let attributes = Attribute::parse_all(&ast.attrs)?;
        let fields = FieldKorok::parse_all(&ast.fields)?;
        Ok(Self {
            ast,
            attributes,
            fields,
        })
    }
}

pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
}
