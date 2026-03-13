use crate::utils::FromMeta;
use codama_errors::IteratorCombineErrors;
use codama_nodes::{
    PdaLinkNode, PdaSeedValueNode, PdaSeedValueValueNode, PdaValueNode, ProgramLinkNode,
};
use codama_syn_helpers::{extensions::*, Meta};

const ATA_SEED_NAMES: [&str; 3] = ["owner", "tokenProgram", "mint"];

pub fn parse_ata_value_node(meta: &Meta) -> syn::Result<PdaValueNode> {
    let pl = meta.assert_directive("ata")?.as_path_list()?;
    let values: Vec<PdaSeedValueValueNode> = pl
        .parse_metas()?
        .iter()
        .map(PdaSeedValueValueNode::from_meta)
        .collect_and_combine_errors()?;

    if values.len() != 3 {
        return Err(meta.error(format!(
            "ata() requires exactly 3 values (owner, tokenProgram, mint), got {}",
            values.len()
        )));
    }

    let seeds = ATA_SEED_NAMES
        .iter()
        .zip(values)
        .map(|(name, value)| PdaSeedValueNode::new(*name, value))
        .collect();

    Ok(PdaValueNode::new(
        PdaLinkNode::new_from_program("associatedToken", ProgramLinkNode::new("associatedToken")),
        seeds,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_value, assert_value_err};
    use codama_nodes::{AccountValueNode, ArgumentValueNode};

    #[test]
    fn ata_with_account_seeds() {
        assert_value!(
            { ata(account("owner"), account("tokenProgram"), account("mint"),) },
            PdaValueNode::new(
                PdaLinkNode::new_from_program(
                    "associatedToken",
                    ProgramLinkNode::new("associatedToken"),
                ),
                vec![
                    PdaSeedValueNode::new("owner", AccountValueNode::new("owner")),
                    PdaSeedValueNode::new("tokenProgram", AccountValueNode::new("tokenProgram")),
                    PdaSeedValueNode::new("mint", AccountValueNode::new("mint")),
                ],
            )
            .into()
        );
    }

    #[test]
    fn ata_with_different_account_names() {
        assert_value!(
            { ata(account("escrow"), account("tokenProgram"), account("mint"),) },
            PdaValueNode::new(
                PdaLinkNode::new_from_program(
                    "associatedToken",
                    ProgramLinkNode::new("associatedToken"),
                ),
                vec![
                    PdaSeedValueNode::new("owner", AccountValueNode::new("escrow")),
                    PdaSeedValueNode::new("tokenProgram", AccountValueNode::new("tokenProgram")),
                    PdaSeedValueNode::new("mint", AccountValueNode::new("mint")),
                ],
            )
            .into()
        );
    }

    #[test]
    fn ata_with_argument_seeds() {
        assert_value!(
            {
                ata(
                    argument("owner"),
                    argument("tokenProgram"),
                    argument("mint"),
                )
            },
            PdaValueNode::new(
                PdaLinkNode::new_from_program(
                    "associatedToken",
                    ProgramLinkNode::new("associatedToken"),
                ),
                vec![
                    PdaSeedValueNode::new("owner", ArgumentValueNode::new("owner")),
                    PdaSeedValueNode::new("tokenProgram", ArgumentValueNode::new("tokenProgram")),
                    PdaSeedValueNode::new("mint", ArgumentValueNode::new("mint")),
                ],
            )
            .into()
        );
    }

    #[test]
    fn ata_with_mixed_seed_types() {
        assert_value!(
            { ata(account("owner"), argument("tokenProgram"), account("mint"),) },
            PdaValueNode::new(
                PdaLinkNode::new_from_program(
                    "associatedToken",
                    ProgramLinkNode::new("associatedToken"),
                ),
                vec![
                    PdaSeedValueNode::new("owner", AccountValueNode::new("owner")),
                    PdaSeedValueNode::new("tokenProgram", ArgumentValueNode::new("tokenProgram")),
                    PdaSeedValueNode::new("mint", AccountValueNode::new("mint")),
                ],
            )
            .into()
        );
    }

    #[test]
    fn ata_empty_args() {
        assert_value_err!(
            { ata() },
            "ata() requires exactly 3 values (owner, tokenProgram, mint), got 0"
        );
    }

    #[test]
    fn ata_wrong_seed_count() {
        assert_value_err!(
            { ata(account("owner"), account("tokenProgram")) },
            "ata() requires exactly 3 values (owner, tokenProgram, mint), got 2"
        );
    }

    #[test]
    fn ata_too_many_seeds() {
        assert_value_err!(
            {
                ata(
                    account("owner"),
                    account("tokenProgram"),
                    account("mint"),
                    account("extra"),
                )
            },
            "ata() requires exactly 3 values (owner, tokenProgram, mint), got 4"
        );
    }

    #[test]
    fn ata_single_seed() {
        assert_value_err!(
            { ata(account("owner")) },
            "ata() requires exactly 3 values (owner, tokenProgram, mint), got 1"
        );
    }
}
