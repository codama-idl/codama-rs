use crate::{
    utils::{FromMeta, MetaConsumer, SetOnce},
    DefaultValueDirective,
};
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, InstructionInputValueNode, TypeNode, ValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};

pub(crate) struct StructFieldMetaConsumer {
    pub metas: Vec<Meta>,
    pub name: SetOnce<CamelCaseString>,
    pub r#type: SetOnce<TypeNode>,
    pub default_value: SetOnce<DefaultValueDirective>,
    pub after: SetOnce<bool>,
}

impl MetaConsumer for StructFieldMetaConsumer {
    fn new(metas: Vec<Meta>) -> Self {
        Self {
            metas,
            name: SetOnce::new("name"),
            r#type: SetOnce::new("type"),
            default_value: SetOnce::new("default_value"),
            after: SetOnce::new("after"),
        }
    }

    fn metas(&self) -> &[Meta] {
        &self.metas
    }

    fn metas_mut(&mut self) -> &mut Vec<Meta> {
        &mut self.metas
    }
}

impl StructFieldMetaConsumer {
    pub fn consume_field(self) -> syn::Result<Self> {
        self.consume_metas(|this, meta| match meta.path_str().as_str() {
            "name" => {
                this.name
                    .set(meta.as_value()?.as_expr()?.as_string()?.into(), meta)?;
                Ok(None)
            }
            "type" => {
                this.r#type
                    .set(TypeNode::from_meta(meta.as_value()?)?, meta)?;
                Ok(None)
            }
            _ => {
                if let Ok(value) = meta.as_expr().and_then(|e| e.as_string()) {
                    this.name.set(value.into(), meta)?;
                    return Ok(None);
                }
                if let Ok(node) = TypeNode::from_meta(&meta) {
                    this.r#type.set(node, meta)?;
                    return Ok(None);
                }
                Ok(Some(meta))
            }
        })
    }

    pub fn consume_default_value(self) -> syn::Result<Self> {
        self.consume_metas(|this, meta| match meta.path_str().as_str() {
            "default_value" | "value" => {
                let directive = DefaultValueDirective::parse_value_nodes_only(&meta)?;
                this.default_value.set(directive, meta)?;
                Ok(None)
            }
            _ => Ok(Some(meta)),
        })
    }

    pub fn consume_argument_default_value(self) -> syn::Result<Self> {
        self.consume_metas(|this, meta| match meta.path_str().as_str() {
            "default_value" | "value" => {
                let directive = DefaultValueDirective::parse(&meta)?;
                this.default_value.set(directive, meta)?;
                Ok(None)
            }
            _ => Ok(Some(meta)),
        })
    }

    pub fn consume_after(self) -> syn::Result<Self> {
        self.consume_metas(|this, meta| match meta.path_str().as_str() {
            "after" => {
                this.after.set(bool::from_meta(&meta)?, meta)?;
                Ok(None)
            }
            _ => Ok(Some(meta)),
        })
    }

    pub fn default_value_strategy(&self) -> Option<DefaultValueStrategy> {
        self.default_value
            .option_ref()
            .and_then(|directive| directive.default_value_strategy)
    }

    pub fn default_value_node(&self) -> Option<ValueNode> {
        self.default_value
            .option_ref()
            .and_then(|directive| ValueNode::try_from(directive.node.clone()).ok())
    }

    pub fn default_instruction_input_value_node(&self) -> Option<InstructionInputValueNode> {
        self.default_value
            .option_ref()
            .map(|directive| directive.node.clone())
    }
}
