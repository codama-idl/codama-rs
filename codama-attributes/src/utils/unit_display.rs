use crate::utils::FromMeta;
use codama_nodes::{NumberDisplayNode, UnitNumberDisplayNode};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses the display of a float or a fixed point. Their scale is static, so the spec allows a unit display only.
/// Any amount display gets the same error, because its decimals cannot apply.
pub fn parse_unit_display(meta: &Meta, nodes: &str) -> syn::Result<UnitNumberDisplayNode> {
    match NumberDisplayNode::from_meta(meta) {
        Ok(NumberDisplayNode::Unit(display)) => Ok(display),
        Err(error) if meta.path_str() != "amount" => Err(error),
        _ => Err(meta.error(format!("{nodes} take a unit display, use unit(...)"))),
    }
}
