use crate::LinkableDictionary;
use codama_nodes::{
    CountNode, DefinedTypeLinkNode, EnumTypeNode, EnumVariantTypeNode, NestedTypeNode,
    NumberFormat, PostOffsetStrategy, PreOffsetStrategy, TypeNode, TypeNodeTrait,
};

/// Computes the fixed serialized byte size of a type, or `None` if the type is
/// variable-length (or its size can't be determined).
///
/// The Rust counterpart of `@codama/visitors-core`'s `getByteSizeVisitor`.
/// `definedTypeLinkNode`s are resolved through `linkables`, scoped to
/// `current_program` (a link's own `program` reference takes precedence); cyclic
/// defined types resolve to `None`.
///
/// This first iteration conservatively returns `None` for the offset/prefix/
/// sentinel wrapper type nodes (`hiddenPrefix`, `hiddenSuffix`, `sizePrefix`,
/// `sentinel`); `None` is always a safe answer (callers treat it as "unknown").
///
/// ```
/// use codama_nodes::{NumberTypeNode, TypeNode, U32};
/// use codama_visitors_core::{get_byte_size, LinkableDictionary};
///
/// let dict = LinkableDictionary::default();
/// let node: TypeNode = NumberTypeNode::le(U32).into();
/// assert_eq!(get_byte_size(&node, &dict, None), Some(4));
/// ```
pub fn get_byte_size(
    node: &TypeNode,
    linkables: &LinkableDictionary,
    current_program: Option<&str>,
) -> Option<usize> {
    ByteSize {
        linkables,
        in_progress: Vec::new(),
    }
    .type_size(node, current_program)
}

struct ByteSize<'a> {
    linkables: &'a LinkableDictionary,
    in_progress: Vec<String>,
}

impl ByteSize<'_> {
    fn type_size(&mut self, node: &TypeNode, program: Option<&str>) -> Option<usize> {
        match node {
            TypeNode::Number(n) => number_size(&n.format),
            TypeNode::PublicKey(_) => Some(32),
            TypeNode::Bytes(_) | TypeNode::String(_) => None,
            TypeNode::Boolean(n) => self.nested(&n.size, program),
            TypeNode::Amount(n) => self.nested(&n.number, program),
            TypeNode::SolAmount(n) => self.nested(&n.number, program),
            TypeNode::DateTime(n) => self.nested(&n.number, program),
            TypeNode::FixedSize(n) => Some(n.size),
            TypeNode::Array(n) => {
                let inner = self.type_size(&n.item, program);
                self.array_like(&n.count, inner, program)
            }
            TypeNode::Set(n) => {
                let inner = self.type_size(&n.item, program);
                self.array_like(&n.count, inner, program)
            }
            TypeNode::Map(n) => {
                let inner = sum([
                    self.type_size(&n.key, program),
                    self.type_size(&n.value, program),
                ]);
                self.array_like(&n.count, inner, program)
            }
            TypeNode::Option(n) => {
                if n.fixed == Some(true) {
                    sum([
                        self.nested(&n.prefix, program),
                        self.type_size(&n.item, program),
                    ])
                } else {
                    None
                }
            }
            TypeNode::RemainderOption(n) => {
                (self.type_size(&n.item, program) == Some(0)).then_some(0)
            }
            TypeNode::ZeroableOption(n) => {
                let item = self.type_size(&n.item, program)?;
                match &n.zero_value {
                    None => Some(item),
                    Some(zero) => {
                        (self.type_size(&zero.r#type, program) == Some(item)).then_some(item)
                    }
                }
            }
            TypeNode::Struct(n) => sum(n
                .fields
                .iter()
                .map(|field| self.type_size(&field.r#type, program))
                .collect::<Vec<_>>()),
            TypeNode::Tuple(n) => sum(n
                .items
                .iter()
                .map(|item| self.type_size(item, program))
                .collect::<Vec<_>>()),
            TypeNode::Enum(n) => self.enum_size(n, program),
            TypeNode::PreOffset(n) => self.offset_size(
                &n.r#type,
                n.offset,
                n.strategy == PreOffsetStrategy::Padded,
                program,
            ),
            TypeNode::PostOffset(n) => self.offset_size(
                &n.r#type,
                n.offset,
                n.strategy == PostOffsetStrategy::Padded,
                program,
            ),
            // Conservatively unknown for now; `None` is always a safe answer.
            TypeNode::HiddenPrefix(_)
            | TypeNode::HiddenSuffix(_)
            | TypeNode::SizePrefix(_)
            | TypeNode::Sentinel(_) => None,
            TypeNode::Link(link) => self.link_size(link, program),
        }
    }

    /// Size of a [`NestedTypeNode`] field by treating it as a plain type.
    fn nested<T: TypeNodeTrait>(
        &mut self,
        node: &NestedTypeNode<T>,
        program: Option<&str>,
    ) -> Option<usize> {
        let type_node: TypeNode = node.clone().into();
        self.type_size(&type_node, program)
    }

    fn array_like(
        &mut self,
        count: &CountNode,
        inner: Option<usize>,
        program: Option<&str>,
    ) -> Option<usize> {
        match (inner, count) {
            (Some(0), CountNode::Prefixed(c)) => self.nested(&c.prefix, program),
            (Some(0), _) => Some(0),
            (_, CountNode::Fixed(c)) if c.value == 0 => Some(0),
            (Some(inner), CountNode::Fixed(c)) => usize::try_from(c.value)
                .ok()
                .and_then(|n| inner.checked_mul(n)),
            _ => None,
        }
    }

    fn enum_size(&mut self, node: &EnumTypeNode, program: Option<&str>) -> Option<usize> {
        let prefix = self.nested(&node.size, program)?;
        let is_scalar = node
            .variants
            .iter()
            .all(|v| matches!(v, EnumVariantTypeNode::Empty(_)));
        if is_scalar {
            return Some(prefix);
        }
        if node.variants.is_empty() {
            return None;
        }
        let first = self.variant_size(&node.variants[0], program);
        let all_same = node
            .variants
            .iter()
            .all(|v| self.variant_size(v, program) == first);
        match first {
            Some(size) if all_same => prefix.checked_add(size),
            _ => None,
        }
    }

    fn variant_size(
        &mut self,
        variant: &EnumVariantTypeNode,
        program: Option<&str>,
    ) -> Option<usize> {
        match variant {
            EnumVariantTypeNode::Empty(_) => Some(0),
            EnumVariantTypeNode::Struct(v) => self.nested(&v.r#struct, program),
            EnumVariantTypeNode::Tuple(v) => self.nested(&v.tuple, program),
        }
    }

    fn offset_size(
        &mut self,
        type_node: &TypeNode,
        offset: i32,
        padded: bool,
        program: Option<&str>,
    ) -> Option<usize> {
        let size = self.type_size(type_node, program)?;
        if padded {
            usize::try_from(offset)
                .ok()
                .and_then(|o| size.checked_add(o))
        } else {
            Some(size)
        }
    }

    fn link_size(&mut self, link: &DefinedTypeLinkNode, program: Option<&str>) -> Option<usize> {
        // Copy the dictionary reference so the lookup borrows it, not `self`.
        let linkables = self.linkables;
        let (name, body, link_program) = {
            let defined_type = linkables.get_defined_type(link, program)?;
            (
                defined_type.name.to_string(),
                (*defined_type.r#type).clone(),
                link.program.as_ref().map(|p| p.name.to_string()),
            )
        };
        if self.in_progress.iter().any(|n| n == &name) {
            return None; // cyclic defined type
        }
        let new_program = link_program.or_else(|| program.map(String::from));
        self.in_progress.push(name);
        let size = self.type_size(&body, new_program.as_deref());
        self.in_progress.pop();
        size
    }
}

fn number_size(format: &NumberFormat) -> Option<usize> {
    use NumberFormat::*;
    Some(match format {
        U8 | I8 => 1,
        U16 | I16 => 2,
        U32 | I32 | F32 => 4,
        U64 | I64 | F64 => 8,
        U128 | I128 => 16,
        ShortU16 => return None,
    })
}

fn sum(values: impl IntoIterator<Item = Option<usize>>) -> Option<usize> {
    let mut total = 0usize;
    for value in values {
        total = total.checked_add(value?)?;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        ArrayTypeNode, BytesTypeNode, DefinedTypeNode, NumberTypeNode, PdaNode, ProgramNode,
        PublicKeyTypeNode, RootNode, StructFieldTypeNode, StructTypeNode, U32, U64, U8,
    };

    fn size(node: impl Into<TypeNode>) -> Option<usize> {
        get_byte_size(&node.into(), &LinkableDictionary::default(), None)
    }

    #[test]
    fn scalars() {
        assert_eq!(size(NumberTypeNode::le(U32)), Some(4));
        assert_eq!(size(NumberTypeNode::le(U64)), Some(8));
        assert_eq!(size(PublicKeyTypeNode::default()), Some(32));
        assert_eq!(size(BytesTypeNode::new()), None);
    }

    #[test]
    fn fixed_array_and_struct() {
        assert_eq!(
            size(ArrayTypeNode::fixed(NumberTypeNode::le(U32), 3)),
            Some(12)
        );
        let s = StructTypeNode::new(vec![
            StructFieldTypeNode::new("a", NumberTypeNode::le(U8)),
            StructFieldTypeNode::new("b", NumberTypeNode::le(U32)),
        ]);
        assert_eq!(size(s), Some(5));
    }

    #[test]
    fn variable_arrays_are_none() {
        // An array with a non-u8 variable item and no fixed count → unknown.
        let bytes_array = ArrayTypeNode::fixed(BytesTypeNode::new(), 4);
        assert_eq!(size(bytes_array), None);
    }

    #[test]
    fn resolves_defined_type_links() {
        let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
        program
            .defined_types
            .push(DefinedTypeNode::new("amount", NumberTypeNode::le(U64)));
        program.pdas.push(PdaNode::new("ignored", vec![]));
        let dict = LinkableDictionary::from_root(&RootNode::new(program));

        let link: TypeNode = codama_nodes::DefinedTypeLinkNode {
            name: "amount".into(),
            program: None,
        }
        .into();
        assert_eq!(get_byte_size(&link, &dict, Some("p")), Some(8));
        // Unknown link → None.
        let missing: TypeNode = codama_nodes::DefinedTypeLinkNode {
            name: "ghost".into(),
            program: None,
        }
        .into();
        assert_eq!(get_byte_size(&missing, &dict, Some("p")), None);
    }
}
