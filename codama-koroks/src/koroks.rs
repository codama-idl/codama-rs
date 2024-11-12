use std::path::Path;

use cargo_toml::Manifest;

use crate::internals::ParsingResult;
use crate::nodes::NumberTypeNode;
use crate::stores::{CrateStore, ModuleStore, RootStore};
use crate::{attributes::Attribute, nodes::TypeNode};

#[derive(Debug)]
pub struct RootKorok<'a> {
    pub crates: Vec<CrateKorok<'a>>,
}

impl<'a> RootKorok<'a> {
    pub fn parse(unparsed_root: &'a RootStore) -> ParsingResult<Self> {
        Ok(Self {
            crates: unparsed_root
                .crates
                .iter()
                .map(CrateKorok::parse)
                .collect::<ParsingResult<_>>()?,
        })
    }
}

#[derive(Debug)]
pub struct CrateKorok<'a> {
    pub file: &'a syn::File,
    pub items: Vec<ItemKorok<'a>>,
    pub manifest: &'a Option<Manifest>,
    pub path: &'a Path,
}

impl<'a> CrateKorok<'a> {
    pub fn parse(unparsed_crate: &'a CrateStore) -> ParsingResult<Self> {
        Ok(Self {
            file: &unparsed_crate.file,
            items: ItemKorok::parse_all(&unparsed_crate.file.items, &unparsed_crate.modules)?,
            manifest: &unparsed_crate.manifest,
            path: &unparsed_crate.path,
        })
    }
}

#[derive(Debug)]
pub enum ItemKorok<'a> {
    FileModule(FileModuleKorok<'a>),
    Module(ModuleKorok<'a>),
    Struct(StructKorok<'a>),
    Enum(EnumKorok<'a>),
    Unsupported(UnsupportedItemKorok<'a>),
}

impl<'a> ItemKorok<'a> {
    pub fn parse(
        item: &'a syn::Item,
        modules: &'a Vec<ModuleStore>,
        item_index: usize,
    ) -> ParsingResult<Self> {
        match item {
            syn::Item::Mod(ast) if ast.content.is_none() => {
                let module = modules.iter().nth(item_index);
                match module {
                    Some(module) => Ok(ItemKorok::FileModule(FileModuleKorok::parse(ast, module)?)),
                    None => Err(syn::Error::new_spanned(
                        ast,
                        "Associated UnparsedModule not found",
                    )
                    .into()),
                }
            }
            syn::Item::Mod(ast) if ast.content.is_some() => {
                Ok(ItemKorok::Module(ModuleKorok::parse(ast, modules)?))
            }
            syn::Item::Struct(ast) => Ok(ItemKorok::Struct(StructKorok::parse(ast)?)),
            syn::Item::Enum(ast) => Ok(ItemKorok::Enum(EnumKorok::parse(ast)?)),
            _ => Ok(ItemKorok::Unsupported(UnsupportedItemKorok { ast: item })),
        }
    }

    pub fn parse_all(
        items: &'a Vec<syn::Item>,
        modules: &'a Vec<ModuleStore>,
    ) -> ParsingResult<Vec<Self>> {
        items
            .iter()
            .enumerate()
            .map(|(item_index, item)| Self::parse(item, modules, item_index))
            .collect()
    }
}

#[derive(Debug)]
pub struct FileModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub file: &'a syn::File,
    pub items: Vec<ItemKorok<'a>>,
    pub path: &'a Path,
}

impl<'a> FileModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, module: &'a ModuleStore) -> ParsingResult<Self> {
        if let Some(_) = ast.content {
            return Err(syn::Error::new_spanned(
                ast,
                "Module has content, it should be parsed with ModuleKorok",
            )
            .into());
        }

        Ok(Self {
            ast,
            file: &module.file,
            items: ItemKorok::parse_all(&module.file.items, &module.modules)?,
            path: &module.path,
        })
    }
}

#[derive(Debug)]
pub struct ModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub items: Vec<ItemKorok<'a>>,
}

impl<'a> ModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, modules: &'a Vec<ModuleStore>) -> ParsingResult<Self> {
        match &ast.content {
            Some(content) => Ok(Self {
                ast,
                items: ItemKorok::parse_all(&content.1, modules)?,
            }),
            None => Err(syn::Error::new_spanned(
                ast,
                "Module has no content, it should be parsed with FileModuleKorok",
            )
            .into()),
        }
    }
}

#[derive(Debug)]
pub struct StructKorok<'a> {
    pub ast: &'a syn::ItemStruct,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: Vec<FieldKorok<'a>>,
}

impl<'a> StructKorok<'a> {
    pub fn parse(ast: &'a syn::ItemStruct) -> ParsingResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldKorok::parse_all(&ast.fields)?,
        })
    }
}

#[derive(Debug)]
pub struct FieldKorok<'a> {
    pub ast: &'a syn::Field,
    pub attributes: Vec<Attribute<'a>>,
    pub base_type: TypeNode,
}

impl<'a> FieldKorok<'a> {
    pub fn parse_all(fields: &'a syn::Fields) -> ParsingResult<Vec<Self>> {
        match fields {
            syn::Fields::Named(f) => f.named.iter().map(Self::parse).collect(),
            syn::Fields::Unnamed(f) => f.unnamed.iter().map(Self::parse).collect(),
            syn::Fields::Unit => Ok(vec![]),
        }
    }

    pub fn parse(ast: &'a syn::Field) -> ParsingResult<Self> {
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

#[derive(Debug)]
pub struct EnumKorok<'a> {
    pub ast: &'a syn::ItemEnum,
    pub attributes: Vec<Attribute<'a>>,
    pub variants: Vec<EnumVariantKorok<'a>>,
}

impl<'a> EnumKorok<'a> {
    pub fn parse(ast: &'a syn::ItemEnum) -> ParsingResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            variants: EnumVariantKorok::parse_all(&ast.variants)?,
        })
    }
}

#[derive(Debug)]
pub struct EnumVariantKorok<'a> {
    pub ast: &'a syn::Variant,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: Vec<FieldKorok<'a>>,
}

impl<'a> EnumVariantKorok<'a> {
    pub fn parse(ast: &'a syn::Variant) -> ParsingResult<Self> {
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

#[derive(Debug)]
pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
}
