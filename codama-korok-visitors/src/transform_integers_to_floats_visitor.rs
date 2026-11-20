use crate::KorokVisitor;
use codama_errors::{CodamaError, CodamaResult};
use codama_koroks::RootKorok;
use codama_nodes::{
    ConstantPdaSeedValue, FloatValueNode, IntegerValueNode, Node, NodeRef, PdaSeedNode,
    PdaSeedValueValue, PdaValuePda, RootNode, TypeNode, ValueNode,
};
use codama_visitors_core::{
    resolve_defined_type_links, visit, IdentityVisitor, LinkableDictionary, NodePath, NodeStack,
    VisitorExt,
};

/// Runs [`transform_integers_to_floats`] on the root node of the korok, if any.
#[derive(Default)]
pub struct TransformIntegersToFloatsVisitor;

impl TransformIntegersToFloatsVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for TransformIntegersToFloatsVisitor {
    fn visit_root(&mut self, korok: &mut RootKorok) -> CodamaResult<()> {
        if let Some(Node::Root(root)) = &korok.node {
            korok.node = Some(transform_integers_to_floats(root)?.into());
        }
        Ok(())
    }
}

/// Turns each integer value whose type resolves to a float type into a float value with the same digits,
/// so a whole number written at a float position, such as `default_value = 1` on an `f32` field, follows
/// spec v2. A value takes the type of the node that holds it:
///
/// - the default value of a struct field takes the type of the field;
/// - the value of a constant, a constant value or a constant PDA seed takes its own type;
/// - a seed value of a PDA value takes the type of the variable seed with the same identifier.
///
/// Types resolve through defined type and PDA links, from the whole root node. A dangling link and a
/// cycle of links keep the integer. Values inside other values, such as the items of an array value,
/// keep their kind.
///
/// ```
/// use codama_korok_visitors::transform_integers_to_floats;
/// use codama_nodes::{
///     AccountNode, FloatTypeNode, FloatValueNode, IntegerValueNode, ProgramNode, RootNode,
///     StructFieldTypeNode, StructTypeNode, F32,
/// };
///
/// let account = |value| {
///     let field = StructFieldTypeNode {
///         default_value: Box::new(Some(value)),
///         ..StructFieldTypeNode::new("bonus", FloatTypeNode::le(F32))
///     };
///     RootNode::new(ProgramNode {
///         accounts: vec![AccountNode::new("config", StructTypeNode::new(vec![field]))],
///         ..ProgramNode::new("exchange", "Exchange11111111111111111111111111111111111")
///     })
/// };
///
/// let root = account(IntegerValueNode::new(1u64).into());
/// let expected = account(FloatValueNode::new(1.0).into());
/// assert_eq!(transform_integers_to_floats(&root).unwrap(), expected);
/// ```
pub fn transform_integers_to_floats(root: &RootNode) -> CodamaResult<RootNode> {
    let stack = NodeStack::default();
    let linkables = LinkableDictionary::default();
    let is_float_at = |path, r#type| is_float_type_at(path, r#type, &linkables);

    let visitor = IdentityVisitor
        .extend(|node, scope| {
            let path = stack.get_path();
            match (node, scope.next(node)?) {
                (NodeRef::StructFieldType(field), Some(Node::StructFieldType(mut new))) => {
                    if let Some(value) = new.default_value.as_mut() {
                        if is_float_at(path, &field.r#type)? {
                            to_float_value(value);
                        }
                    }
                    Ok(Some(new.into()))
                }
                (NodeRef::ConstantValue(constant), Some(Node::ConstantValue(mut new))) => {
                    if is_float_at(path, &constant.r#type)? {
                        to_float_value(&mut new.value);
                    }
                    Ok(Some(new.into()))
                }
                (NodeRef::Constant(constant), Some(Node::Constant(mut new))) => {
                    if is_float_at(path, &constant.r#type)? {
                        to_float_value(&mut new.value);
                    }
                    Ok(Some(new.into()))
                }
                (NodeRef::ConstantPdaSeed(seed), Some(Node::ConstantPdaSeed(mut new))) => {
                    if let ConstantPdaSeedValue::Integer(integer) = new.value.as_ref() {
                        if is_float_at(path, &seed.r#type)? {
                            *new.value = to_float(integer).into();
                        }
                    }
                    Ok(Some(new.into()))
                }
                (NodeRef::PdaValue(value), Some(Node::PdaValue(mut new))) => {
                    let pda_path = match value.pda.as_ref() {
                        PdaValuePda::Pda(pda) => {
                            let mut pda_path = path;
                            pda_path.push(pda);
                            Some(pda_path)
                        }
                        PdaValuePda::PdaLink(link) => {
                            let mut link_path = path;
                            link_path.push(link);
                            linkables.get_path(&link_path)
                        }
                    };
                    let Some(pda_path) = pda_path else {
                        return Ok(Some(new.into()));
                    };
                    let Some(NodeRef::Pda(pda)) = pda_path.last() else {
                        unreachable!("a PDA link resolves to a PDA");
                    };
                    for seed in &mut new.seeds {
                        let PdaSeedValueValue::Integer(integer) = seed.value.as_ref() else {
                            continue;
                        };
                        let variable_seed =
                            pda.seeds.iter().find_map(|candidate| match candidate {
                                PdaSeedNode::Variable(candidate)
                                    if candidate.identifier == seed.identifier =>
                                {
                                    Some(candidate)
                                }
                                _ => None,
                            });
                        let Some(variable_seed) = variable_seed else {
                            continue;
                        };
                        let mut seed_path = pda_path.clone();
                        seed_path.push(variable_seed);
                        if is_float_at(seed_path, &variable_seed.r#type)? {
                            *seed.value = to_float(integer).into();
                        }
                    }
                    Ok(Some(new.into()))
                }
                (_, new) => Ok(new),
            }
        })
        .record_node_stack(&stack)
        .record_linkables_on_first_visit(&linkables)
        .non_null();

    match visit(&visitor, root)? {
        Node::Root(root) => Ok(root),
        _ => unreachable!("the pass changes values only"),
    }
}

/// Whether `type`, the child of the last node of `path`, resolves to a float type. A dangling link or a
/// cycle of links does not resolve.
fn is_float_type_at<'a>(
    mut path: NodePath<'a>,
    r#type: &'a TypeNode,
    linkables: &LinkableDictionary<'a>,
) -> CodamaResult<bool> {
    path.push(r#type);
    match resolve_defined_type_links(&path, linkables) {
        Ok(resolved) => Ok(matches!(
            resolved.and_then(|resolved| resolved.last()),
            Some(NodeRef::FloatType(_))
        )),
        Err(CodamaError::LinkedNodeNotFound { .. }) => Ok(false),
        Err(error) => Err(error),
    }
}

fn to_float_value(value: &mut ValueNode) {
    if let ValueNode::Integer(integer) = value {
        *value = to_float(integer).into();
    }
}

fn to_float(integer: &IntegerValueNode) -> FloatValueNode {
    FloatValueNode {
        value: integer.value.into(),
        plugins: integer.plugins.clone(),
    }
}
