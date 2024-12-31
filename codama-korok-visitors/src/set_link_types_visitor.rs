use crate::KorokVisitor;
use codama_nodes::DefinedTypeLinkNode;
use codama_syn_helpers::extensions::*;

#[derive(Default)]
pub struct SetLinkTypesVisitor;

impl SetLinkTypesVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl KorokVisitor for SetLinkTypesVisitor {
    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> syn::Result<()> {
        if korok.node.is_some() {
            return Ok(());
        }
        if let syn::Type::Path(tp) = &korok.ast.ty {
            korok.set_type_node(DefinedTypeLinkNode::new(tp.path.last_str()).into())
        }
        Ok(())
    }
}
