use crate::{validation_item, LogLevel, ValidationItem};
use codama_nodes::{Identifier, InstructionNode, NodeRef, Path};
use codama_visitors_core::{
    resolve_type_path, LinkableDictionary, NodePath, ResolveTypePathError, ResolveTypePathOptions,
};

/// A contextual value that refers to its enclosing instruction, borrowed from the node that holds it.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ContextualValue<'v> {
    /// The data of the instruction at a path, like a `dataValueNode`.
    Data(&'v Path),
    /// The address of an account of the instruction, like an `accountValueNode`.
    Account(&'v Identifier),
    /// The data of an account of the instruction, at a path if any, like an `accountDataValueNode`.
    AccountData(&'v Identifier, Option<&'v Path>),
}

/// The items of a contextual value referring to its enclosing instruction, like `getContextualValueItems`
/// in the JavaScript library.
///
/// The data path of a data value must resolve within the instruction data, and the account of an
/// account or account data value must be an account of the instruction, with an account link for the
/// latter so its data path can resolve within the data of the linked account. Paths may cross account
/// links. Values outside instructions are skipped, and so are links that cannot be resolved, which are
/// reported on their own.
///
/// `item_path` is the path the items point to, ending with the value or the node that holds it.
pub(crate) fn get_contextual_value_items<'a>(
    value: ContextualValue,
    item_path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> Vec<ValidationItem<'a>> {
    let Some(instruction_path) = item_path.until_last::<InstructionNode>() else {
        return vec![];
    };
    let instruction: &InstructionNode = instruction_path
        .last()
        .and_then(|node| node.try_into().ok())
        .expect("until_last ends with an instruction");
    let error =
        |message: String| vec![validation_item(LogLevel::Error, message, item_path.clone())];

    let (account_identifier, data_path) = match value {
        ContextualValue::Data(path) => {
            let Some(data) = instruction.data.as_ref() else {
                return error(format!(
                    "Data path \"{path}\" cannot be resolved as instruction \"{}\" has no data.",
                    instruction.identifier
                ));
            };
            let mut source = instruction_path.clone();
            source.push(data);
            return get_path_items(&source, path, item_path, linkables);
        }
        ContextualValue::Account(identifier) => (identifier, None),
        ContextualValue::AccountData(identifier, path) => (identifier, Some(path)),
    };

    let Some(account) = instruction
        .accounts
        .iter()
        .find(|account| account.identifier == *account_identifier)
    else {
        return error(format!(
            "Account \"{account_identifier}\" is not an account of instruction \"{}\".",
            instruction.identifier
        ));
    };
    // An account value only needs the account to exist.
    let Some(data_path) = data_path else {
        return vec![];
    };
    // An account data value also needs a link to read the data of the account from.
    let Some(account_link) = account.account_link.as_ref() else {
        return error(format!(
            "Account \"{account_identifier}\" has no account link, so its data cannot be read."
        ));
    };
    // Without a path, it reads the whole data, which always resolves.
    let Some(path) = data_path else {
        return vec![];
    };
    let mut link_path = instruction_path.clone();
    link_path.push(account);
    link_path.push(account_link);
    let Some(mut linked_account_path) = linkables.get_path(&link_path) else {
        return vec![];
    };
    let linked_account: &codama_nodes::AccountNode = linked_account_path
        .last()
        .and_then(|node| node.try_into().ok())
        .expect("an account link resolves to an account");
    linked_account_path.push(&*linked_account.data);
    get_path_items(&linked_account_path, path, item_path, linkables)
}

/// The items of a path that cannot be resolved against the type at the end of `source`.
fn get_path_items<'a>(
    source: &NodePath<'a>,
    path: &Path,
    item_path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> Vec<ValidationItem<'a>> {
    let options = ResolveTypePathOptions {
        follow_account_links: true,
    };
    match resolve_type_path(source, path, linkables, options) {
        Ok(_) | Err(ResolveTypePathError::LinkedNodeNotFound { .. }) => vec![],
        Err(error) => {
            let cause = codama_errors::CodamaError::from(error);
            vec![
                validation_item(LogLevel::Error, cause.to_string(), item_path.clone())
                    .with_cause(cause),
            ]
        }
    }
}

/// Whether `node` is a public key with an account link, which a `.data` segment would cross.
pub(crate) fn is_linked_public_key(node: Option<NodeRef>) -> bool {
    matches!(node, Some(NodeRef::PublicKeyType(public_key)) if public_key.account_link.is_some())
}
