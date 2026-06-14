use codama_nodes::{Node, RootNode, TypeNode};
use codama_visitors_core::{
    bottom_up_transformer, get_byte_size, LinkableDictionary, TransformRule, TransformVisitor,
};

/// Sets `size` on every account whose data has a fixed byte size.
///
/// The Rust counterpart of `@codama/visitors`' `setFixedAccountSizesVisitor`. It
/// is root-coupled (unlike the other visitors, it takes and returns a
/// [`RootNode`] rather than a transformer): it first builds a
/// [`LinkableDictionary`] from the whole tree so `definedTypeLinkNode`s in
/// account data can be resolved, then computes each account's size via
/// [`get_byte_size`] using the account's own program scope. Accounts whose data
/// is variable-length are left unchanged.
///
/// ```
/// use codama_nodes::{AccountNode, NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U32};
/// use codama_visitors::set_fixed_account_sizes;
///
/// let data = StructTypeNode::new(vec![StructFieldTypeNode::new("value", NumberTypeNode::le(U32))]);
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.accounts.push(AccountNode::new("counter", data));
/// let root = set_fixed_account_sizes(RootNode::new(program));
/// assert_eq!(root.program.accounts[0].size, Some(4));
/// ```
pub fn set_fixed_account_sizes(root: RootNode) -> RootNode {
    let linkables = LinkableDictionary::from_root(&root);
    let mut visitor = bottom_up_transformer(vec![TransformRule::new(
        "[accountNode]",
        move |node, path| {
            let Node::Account(mut account) = node else {
                return node;
            };
            let program = path
                .program()
                .and_then(|descriptor| descriptor.name.as_ref())
                .map(|name| name.as_ref());
            let data: TypeNode = account.data.clone().into();
            if let Some(size) = get_byte_size(&data, &linkables, program) {
                if let Ok(size) = u64::try_from(size) {
                    account.size = Some(size);
                }
            }
            Node::Account(account)
        },
    )]);
    visitor.visit_root(root)
}
