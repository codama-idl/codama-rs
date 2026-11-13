use crate::{LinkableDictionary, NodePath};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{DefinedTypeNode, NodeRef, TypeNode, ValueNode};

/// Whether a concrete value is a value of the type at the end of `type_path`, structurally: its kind
/// fits the type and so do its children, such as the items of an array, the fields of a struct or the
/// data of an enum variant. Like `isValueNodeOfType` in the JavaScript library.
///
/// Links to defined types are followed, which fails with `LinkedNodeNotFound` when a link points to no
/// recorded node. Ranges are not checked, so `256` is a value of a `u8`. Injected and constant values
/// are not concrete, so they are never values of a type here. Numbers accept integers and floats also
/// accept floats, a struct value must provide every field without a default value, and an enum value
/// must name a variant of the enum and carry its data, if any. The enum that an enum value links to is
/// not compared to the type, as values are matched by their structure only.
///
/// ```
/// use codama_nodes::{IntegerTypeNode, NodeRef, NoneValueNode, OptionTypeNode, U64};
/// use codama_visitors_core::{is_value_node_of_type, LinkableDictionary, NodePath};
///
/// let r#type = OptionTypeNode::new(IntegerTypeNode::le(U64));
/// let path = NodePath::from(vec![NodeRef::from(&r#type)]);
/// let linkables = LinkableDictionary::default();
/// assert!(is_value_node_of_type(&NoneValueNode::new().into(), &path, &linkables).unwrap());
/// ```
pub fn is_value_node_of_type<'a>(
    value: &ValueNode,
    type_path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> CodamaResult<bool> {
    let Some(resolved) = resolve_defined_type_links(type_path, linkables)? else {
        return Ok(false);
    };
    let is_child_of_type = |child_value: &ValueNode, child_type: &'a TypeNode| {
        let mut child_path = resolved.clone();
        child_path.push(child_type);
        is_value_node_of_type(child_value, &child_path, linkables)
    };
    let all = |checks: &mut dyn Iterator<Item = CodamaResult<bool>>| -> CodamaResult<bool> {
        for check in checks {
            if !check? {
                return Ok(false);
            }
        }
        Ok(true)
    };

    Ok(
        match (resolved.last().expect("a type path is never empty"), value) {
            (NodeRef::BooleanType(_), ValueNode::Boolean(_)) => true,
            (NodeRef::BytesType(_), ValueNode::Bytes(_)) => true,
            (
                NodeRef::DateTimeType(_)
                | NodeRef::DurationType(_)
                | NodeRef::FixedPointType(_)
                | NodeRef::IntegerType(_),
                ValueNode::Integer(_),
            ) => true,
            (NodeRef::FloatType(_), ValueNode::Float(_) | ValueNode::Integer(_)) => true,
            (NodeRef::PublicKeyType(_), ValueNode::PublicKey(_)) => true,
            (NodeRef::StringType(_), ValueNode::String(_)) => true,
            (NodeRef::ArrayType(r#type), ValueNode::Array(array)) => all(&mut array
                .items
                .iter()
                .map(|item| is_child_of_type(item, &r#type.item)))?,
            (NodeRef::SetType(r#type), ValueNode::Set(set)) => all(&mut set
                .items
                .iter()
                .map(|item| is_child_of_type(item, &r#type.item)))?,
            (NodeRef::TupleType(r#type), ValueNode::Tuple(tuple)) => {
                tuple.items.len() == r#type.items.len()
                    && all(&mut tuple
                        .items
                        .iter()
                        .zip(&r#type.items)
                        .map(|(item, item_type)| is_child_of_type(item, item_type)))?
            }
            (NodeRef::MapType(r#type), ValueNode::Map(map)) => {
                all(&mut map.entries.iter().map(|entry| {
                    Ok(is_child_of_type(&entry.key, &r#type.key)?
                        && is_child_of_type(&entry.value, &r#type.value)?)
                }))?
            }
            (
                NodeRef::OptionType(_)
                | NodeRef::RemainderOptionType(_)
                | NodeRef::ZeroableOptionType(_),
                ValueNode::None(_),
            ) => true,
            (NodeRef::OptionType(r#type), ValueNode::Some(some)) => {
                is_child_of_type(&some.value, &r#type.item)?
            }
            (NodeRef::RemainderOptionType(r#type), ValueNode::Some(some)) => {
                is_child_of_type(&some.value, &r#type.item)?
            }
            (NodeRef::ZeroableOptionType(r#type), ValueNode::Some(some)) => {
                is_child_of_type(&some.value, &r#type.item)?
            }
            (NodeRef::StructType(r#type), ValueNode::Struct(r#struct)) => {
                let has_valid_fields = all(&mut r#struct.fields.iter().map(|field_value| {
                    match r#type
                        .fields
                        .iter()
                        .find(|field_type| field_type.identifier == field_value.identifier)
                    {
                        Some(field_type) => {
                            is_child_of_type(&field_value.value, &field_type.r#type)
                        }
                        None => Ok(false),
                    }
                }))?;
                let has_required_fields = r#type.fields.iter().all(|field_type| {
                    field_type.default_value.is_some()
                        || r#struct
                            .fields
                            .iter()
                            .any(|field_value| field_value.identifier == field_type.identifier)
                });
                has_valid_fields && has_required_fields
            }
            (NodeRef::EnumType(r#type), ValueNode::Enum(r#enum)) => {
                let Some(variant) = r#type
                    .variants
                    .iter()
                    .find(|variant| variant.identifier == r#enum.variant)
                else {
                    return Ok(false);
                };
                match (variant.data.as_ref(), r#enum.value.as_ref()) {
                    (None, value) => value.is_none(),
                    (Some(_), None) => false,
                    (Some(data), Some(value)) => is_child_of_type(value, data)?,
                }
            }
            _ => false,
        },
    )
}

/// Follows links to defined types until a type that is not a link, or `None` for alias cycles, such as
/// `a = b` and `b = a`, which have no value.
fn resolve_defined_type_links<'a>(
    type_path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> CodamaResult<Option<NodePath<'a>>> {
    let mut current = type_path.clone();
    let mut followed_types: Vec<*const DefinedTypeNode> = vec![];
    while let Some(NodeRef::DefinedTypeLink(_)) = current.last() {
        let mut defined_type_path = linkables.get_path_or_err(&current)?;
        let defined_type: &DefinedTypeNode = defined_type_path
            .last()
            .and_then(|node| node.try_into().ok())
            .ok_or_else(|| CodamaError::LinkedNodeNotFound {
                path: current.to_string(),
            })?;
        if followed_types.contains(&(defined_type as *const _)) {
            return Ok(None);
        }
        followed_types.push(defined_type);
        defined_type_path.push(&*defined_type.r#type);
        current = defined_type_path;
    }
    Ok(Some(current))
}
