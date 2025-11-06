use crate::utils::{FromMeta, SetOnce};
use codama_errors::IteratorCombineErrors;
use codama_nodes::{CamelCaseString, PdaLinkNode, PdaSeedValueNode, PdaValueNode, ProgramLinkNode};
use codama_syn_helpers::{extensions::*, Meta, PathList};

impl FromMeta for PdaValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("pda")?.as_path_list()?;
        let mut name = SetOnce::<CamelCaseString>::new("name");
        let mut program = SetOnce::<CamelCaseString>::new("program");
        let mut seeds = SetOnce::<Vec<PdaSeedValueNode>>::new("seeds");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => name.set(meta.as_value()?.as_expr()?.as_string()?.into(), meta),
            "program" => program.set(meta.as_value()?.as_expr()?.as_string()?.into(), meta),
            "seeds" => seeds.set(parse_seed_value_nodes(meta.as_path_list()?)?, meta),
            _ => {
                if let Ok(seed_name) = meta.as_expr().and_then(|e| e.as_string()) {
                    return name.set(seed_name.into(), meta);
                }
                if let Meta::Expr(syn::Expr::Array(array)) = meta {
                    return seeds.set(parse_seed_value_nodes(&array.as_path_list())?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        let pda_link_node = PdaLinkNode {
            name: name.take(meta)?,
            program: program.option().map(ProgramLinkNode::new),
        };

        Ok(PdaValueNode::new(
            pda_link_node,
            seeds.option().unwrap_or_default(),
        ))
    }
}

fn parse_seed_value_nodes(pl: &PathList) -> syn::Result<Vec<PdaSeedValueNode>> {
    pl.parse_metas()?
        .iter()
        .map(PdaSeedValueNode::from_meta)
        .collect_and_combine_errors()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_value, assert_value_err};
    use codama_nodes::AccountValueNode;

    #[test]
    fn explicit() {
        assert_value!(
            { pda(name = "token", seeds = []) },
            PdaValueNode::new(PdaLinkNode::new("token"), vec![]).into()
        );
    }

    #[test]
    fn implicit() {
        assert_value!(
            {
                pda(
                    "token",
                    [account("mint"), seed("owner", account("authority"))],
                )
            },
            PdaValueNode::new(
                PdaLinkNode::new("token"),
                vec![
                    PdaSeedValueNode::new("mint", AccountValueNode::new("mint")),
                    PdaSeedValueNode::new("owner", AccountValueNode::new("authority"))
                ],
            )
            .into()
        );
    }

    #[test]
    fn implicit_empty() {
        assert_value!(
            { pda("token", []) },
            PdaValueNode::new(PdaLinkNode::new("token"), vec![]).into()
        );
    }

    #[test]
    fn name_only() {
        assert_value!(
            { pda("token") },
            PdaValueNode::new(PdaLinkNode::new("token"), vec![]).into()
        );
    }

    #[test]
    fn missing_name() {
        assert_value_err!({ pda(seeds = []) }, "name is missing");
    }

    #[test]
    fn invalid_name_attribute() {
        assert_value_err!({ pda(banana = "token") }, "unrecognized attribute");
    }
}
