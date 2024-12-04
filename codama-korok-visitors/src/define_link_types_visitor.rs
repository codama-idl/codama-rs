use crate::KorokVisitor;
use codama_nodes::DefinedTypeLinkNode;
use codama_syn_helpers::syn_traits::*;

pub struct DefineLinkTypesVisitor {}

impl DefineLinkTypesVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl KorokVisitor for DefineLinkTypesVisitor {
    fn visit_type(&mut self, korok: &mut codama_koroks::TypeKorok) {
        if korok.node.is_some() {
            return;
        }
        korok.node = match korok.ast {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                Some(DefinedTypeLinkNode::new(path.last_str()).into())
            }
            _ => None,
        };
    }
}
