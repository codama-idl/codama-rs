use crate::utils::{FromMeta, MetaConsumer, SetOnce};
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, InstructionInputValueNode, TypeNode, ValueNode,
};
use codama_syn_helpers::Meta;

pub(crate) struct StructFieldMetaConsumer {
    pub metas: Vec<Meta>,
    pub name: SetOnce<CamelCaseString>,
    pub r#type: SetOnce<TypeNode>,
    pub default_value: SetOnce<ValueNode>,
    pub argument_default_value: SetOnce<InstructionInputValueNode>,
    pub default_value_strategy: SetOnce<DefaultValueStrategy>,
    pub after: SetOnce<bool>,
}

impl MetaConsumer for StructFieldMetaConsumer {
    fn new(metas: Vec<Meta>) -> Self {
        Self {
            metas,
            name: SetOnce::new("name"),
            r#type: SetOnce::new("type"),
            default_value: SetOnce::new("default_value"),
            argument_default_value: SetOnce::new("default_value"),
            default_value_strategy: SetOnce::new("default_value_strategy"),
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
                this.name.set(String::from_meta(&meta)?.into(), meta)?;
                Ok(None)
            }
            "type" => {
                let node = TypeNode::from_meta(&meta.as_path_value()?.value)?;
                this.r#type.set(node, meta)?;
                Ok(None)
            }
            "default_value_omitted" => {
                meta.as_path()?;
                this.default_value_strategy
                    .set(DefaultValueStrategy::Omitted, meta)?;
                Ok(None)
            }
            _ => {
                if let Ok(value) = String::from_meta(&meta) {
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
            "default_value" => {
                let node = ValueNode::from_meta(&meta.as_path_value()?.value)?;
                this.default_value.set(node, meta)?;
                Ok(None)
            }
            _ => Ok(Some(meta)),
        })
    }

    pub fn consume_argument_default_value(self) -> syn::Result<Self> {
        self.consume_metas(|this, meta| match meta.path_str().as_str() {
            "default_value" => {
                let node = InstructionInputValueNode::from_meta(&meta.as_path_value()?.value)?;
                this.argument_default_value.set(node, meta)?;
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
}
