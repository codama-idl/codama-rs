use cargo_toml::Manifest;
use codama_errors::CodamaResult;
use codama_nodes::{
    EnumTypeNode, EnumVariantTypeNode, Node, RegisteredTypeNode, StructFieldTypeNode,
    StructTypeNode, TupleTypeNode, TypeNode,
};
use std::path::Path;

use crate::attributes::Attribute;
use crate::stores::{CrateStore, ModuleStore, RootStore};

#[derive(Debug)]
pub struct RootKorok<'a> {
    pub crates: Vec<CrateKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> RootKorok<'a> {
    pub fn parse(root_store: &'a RootStore) -> CodamaResult<Self> {
        Ok(Self {
            crates: root_store
                .crates
                .iter()
                .map(CrateKorok::parse)
                .collect::<CodamaResult<_>>()?,
            node: None,
        })
    }

    pub fn first_item(&self) -> &ItemKorok {
        &self.crates[0].items[0]
    }
}

#[derive(Debug)]
pub struct CrateKorok<'a> {
    pub file: &'a syn::File,
    pub items: Vec<ItemKorok<'a>>,
    pub manifest: &'a Option<Manifest>,
    pub node: Option<Node>,
    pub path: &'a Path,
}

impl<'a> CrateKorok<'a> {
    pub fn parse(crate_store: &'a CrateStore) -> CodamaResult<Self> {
        Ok(Self {
            file: &crate_store.file,
            items: ItemKorok::parse_all(&crate_store.file.items, &crate_store.modules)?,
            manifest: &crate_store.manifest,
            node: None,
            path: &crate_store.path,
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
    ) -> CodamaResult<Self> {
        match item {
            syn::Item::Mod(ast) if ast.content.is_none() => {
                let module = modules.iter().nth(item_index);
                match module {
                    Some(module) => Ok(ItemKorok::FileModule(FileModuleKorok::parse(ast, module)?)),
                    None => {
                        Err(syn::Error::new_spanned(ast, "Associated ModuleStore not found").into())
                    }
                }
            }
            syn::Item::Mod(ast) if ast.content.is_some() => {
                Ok(ItemKorok::Module(ModuleKorok::parse(ast, modules)?))
            }
            syn::Item::Struct(ast) => Ok(ItemKorok::Struct(StructKorok::parse(ast)?)),
            syn::Item::Enum(ast) => Ok(ItemKorok::Enum(EnumKorok::parse(ast)?)),
            _ => Ok(ItemKorok::Unsupported(UnsupportedItemKorok {
                ast: item,
                node: None,
            })),
        }
    }

    pub fn parse_all(
        items: &'a Vec<syn::Item>,
        modules: &'a Vec<ModuleStore>,
    ) -> CodamaResult<Vec<Self>> {
        items
            .iter()
            .enumerate()
            .map(|(item_index, item)| Self::parse(item, modules, item_index))
            .collect()
    }

    pub fn node(&self) -> Option<Node> {
        match self {
            ItemKorok::Struct(k) => k.node.clone(),
            ItemKorok::Enum(k) => k.node.clone(),
            ItemKorok::FileModule(k) => k.node.clone(),
            ItemKorok::Module(k) => k.node.clone(),
            ItemKorok::Unsupported(k) => k.node.clone(),
        }
    }
}

#[derive(Debug)]
pub struct FileModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub file: &'a syn::File,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
    pub path: &'a Path,
}

impl<'a> FileModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, module: &'a ModuleStore) -> CodamaResult<Self> {
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
            node: None,
        })
    }
}

#[derive(Debug)]
pub struct ModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> ModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, modules: &'a Vec<ModuleStore>) -> CodamaResult<Self> {
        match &ast.content {
            Some(content) => Ok(Self {
                ast,
                items: ItemKorok::parse_all(&content.1, modules)?,
                node: None,
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
    pub fields: FieldsKorok<'a>,
    pub node: Option<Node>,
}

impl<'a> StructKorok<'a> {
    pub fn parse(ast: &'a syn::ItemStruct) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldsKorok::parse(&ast.fields)?,
            node: None,
        })
    }
}

#[derive(Debug)]
pub struct FieldsKorok<'a> {
    pub ast: &'a syn::Fields,
    pub all: Vec<FieldKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> FieldsKorok<'a> {
    pub fn parse(ast: &'a syn::Fields) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            all: match ast {
                syn::Fields::Named(f) => f.named.iter().map(FieldKorok::parse).collect(),
                syn::Fields::Unnamed(f) => f.unnamed.iter().map(FieldKorok::parse).collect(),
                syn::Fields::Unit => Ok(vec![]),
            }?,
            node: None,
        })
    }

    pub fn all_have_nodes(&self) -> bool {
        self.all.iter().all(|field| field.node.is_some())
    }

    pub fn create_struct_node(&self) -> StructTypeNode {
        let fields = self
            .all
            .iter()
            .filter_map(|field| match &field.node {
                Some(Node::Type(RegisteredTypeNode::StructField(field))) => Some(field.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        StructTypeNode::new(fields)
    }

    pub fn create_tuple_node(&self) -> TupleTypeNode {
        let items = self
            .all
            .iter()
            .filter_map(|f| TypeNode::try_from(f.node.clone()).ok())
            .collect::<Vec<_>>();
        TupleTypeNode::new(items)
    }
}

#[derive(Debug)]
pub struct FieldKorok<'a> {
    pub ast: &'a syn::Field,
    pub attributes: Vec<Attribute<'a>>,
    pub node: Option<Node>,
    pub r#type: TypeKorok<'a>,
}

impl<'a> FieldKorok<'a> {
    pub fn parse(ast: &'a syn::Field) -> CodamaResult<Self> {
        let attributes = Attribute::parse_all(&ast.attrs)?;
        Ok(Self {
            ast,
            attributes,
            node: None,
            r#type: TypeKorok::new(&ast.ty),
        })
    }

    pub fn create_type_node(&self) -> Option<RegisteredTypeNode> {
        match &self.r#type.node {
            Some(Node::Type(node)) => match &self.ast.ident {
                Some(ident) => match TypeNode::try_from(node.clone()) {
                    Ok(node) => Some(StructFieldTypeNode::new(ident.to_string(), node).into()),
                    Err(_) => None,
                },
                None => Some(node.clone().into()),
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct TypeKorok<'a> {
    pub ast: &'a syn::Type,
    pub node: Option<Node>,
}

impl<'a> TypeKorok<'a> {
    pub fn new(ast: &'a syn::Type) -> Self {
        Self { ast, node: None }
    }
}

#[derive(Debug)]
pub struct EnumKorok<'a> {
    pub ast: &'a syn::ItemEnum,
    pub attributes: Vec<Attribute<'a>>,
    pub node: Option<Node>,
    pub variants: Vec<EnumVariantKorok<'a>>,
}

impl<'a> EnumKorok<'a> {
    pub fn parse(ast: &'a syn::ItemEnum) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            node: None,
            variants: EnumVariantKorok::parse_all(&ast.variants)?,
        })
    }

    pub fn all_variants_have_nodes(&self) -> bool {
        self.variants.iter().all(|field| field.node.is_some())
    }

    pub fn create_enum_node(&self) -> EnumTypeNode {
        let variants = self
            .variants
            .iter()
            .filter_map(|variant| match &variant.node {
                Some(Node::Type(RegisteredTypeNode::EnumEmptyVariant(node))) => {
                    Some(EnumVariantTypeNode::Empty(node.clone()))
                }
                Some(Node::Type(RegisteredTypeNode::EnumTupleVariant(node))) => {
                    Some(EnumVariantTypeNode::Tuple(node.clone()))
                }
                Some(Node::Type(RegisteredTypeNode::EnumStructVariant(node))) => {
                    Some(EnumVariantTypeNode::Struct(node.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        EnumTypeNode::new(variants)
    }
}

#[derive(Debug)]
pub struct EnumVariantKorok<'a> {
    pub ast: &'a syn::Variant,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: FieldsKorok<'a>,
    pub node: Option<Node>,
}

impl<'a> EnumVariantKorok<'a> {
    pub fn parse(ast: &'a syn::Variant) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldsKorok::parse(&ast.fields)?,
            node: None,
        })
    }

    pub fn parse_all(
        variants: &'a syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> CodamaResult<Vec<Self>> {
        variants.iter().map(Self::parse).collect()
    }
}

#[derive(Debug)]
pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
    pub node: Option<Node>,
}
