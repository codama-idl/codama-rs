use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, DiscriminatorNode, FieldDiscriminatorNode,
    NestedTypeNodeTrait, Node, StructTypeNode, ValueNode,
};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// Which data field identifies an account and the constant value it holds.
#[derive(Debug, Clone)]
pub struct AccountDiscriminator {
    field: String,
    value: ValueNode,
    offset: u64,
}

impl AccountDiscriminator {
    /// Use `field` as the discriminator, fixed to `value`, at byte offset 0.
    pub fn new(field: impl Into<String>, value: impl Into<ValueNode>) -> Self {
        Self {
            field: field.into(),
            value: value.into(),
            offset: 0,
        }
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }
}

/// For each selected account, marks one of its data fields as the discriminator:
/// the field's default value is fixed to the given value (strategy `omitted`)
/// and a `fieldDiscriminatorNode` is prepended to the account's discriminators.
///
/// The Rust counterpart of `@codama/visitors`' `setAccountDiscriminatorFromFieldVisitor`,
/// built on [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
///
/// ```
/// use codama_nodes::{AccountNode, NumberTypeNode, NumberValueNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U8};
/// use codama_visitors::{set_account_discriminator_from_field, AccountDiscriminator, TransformVisitor};
///
/// let data = StructTypeNode::new(vec![StructFieldTypeNode::new("tag", NumberTypeNode::le(U8))]);
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.accounts.push(AccountNode::new("counter", data));
/// let root = RootNode::new(program);
///
/// let mut visitor = set_account_discriminator_from_field([(
///     "counter",
///     AccountDiscriminator::new("tag", NumberValueNode::new(1u8)),
/// )]);
/// let _root = visitor.visit_root(root);
/// ```
///
/// Note: if the named field is absent, the account is left unchanged (upstream
/// throws).
pub fn set_account_discriminator_from_field<S: Into<String>>(
    map: impl IntoIterator<Item = (S, AccountDiscriminator)>,
) -> BottomUpTransformer {
    let rules = map
        .into_iter()
        .map(|(selector, discriminator)| {
            TransformRule::new(selector.into(), move |node, _path| {
                set_discriminator_from_field(node, discriminator.clone())
            })
        })
        .collect();
    bottom_up_transformer(rules)
}

fn set_discriminator_from_field(node: Node, discriminator: AccountDiscriminator) -> Node {
    let Node::Account(mut account) = node else {
        return node;
    };

    let field_name = CamelCaseString::new(&discriminator.field);
    let Some(index) = account
        .data
        .get_nested_type_node()
        .fields
        .iter()
        .position(|f| f.name == field_name)
    else {
        return Node::Account(account); // field not found; leave unchanged
    };

    let value = discriminator.value;
    account.data = account
        .data
        .map_nested_type_node(move |mut data: StructTypeNode| {
            data.fields[index].default_value = Box::new(Some(value));
            data.fields[index].default_value_strategy = Some(DefaultValueStrategy::Omitted);
            data
        });
    account.discriminators.insert(
        0,
        DiscriminatorNode::Field(FieldDiscriminatorNode::new(
            discriminator.field,
            discriminator.offset,
        )),
    );

    Node::Account(account)
}
