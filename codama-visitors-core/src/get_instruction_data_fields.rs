use crate::{LinkableDictionary, NodePath};
use codama_errors::CodamaResult;
use codama_nodes::{DefinedTypeNode, InstructionNode, Path, StructFieldTypeNode, TypeNode};

/// A struct field within the data of an instruction, with its full path.
#[derive(Debug, Clone, PartialEq)]
pub struct InstructionDataField<'a> {
    /// The field itself.
    pub field: &'a StructFieldTypeNode,
    /// The path of the field node, from the start of the instruction path, such as
    /// `[root, program, instruction, struct, field]`. A field within a linked defined type has a path
    /// through that defined type, such as `[root, program, definedType, struct, field]`.
    pub field_path: NodePath<'a>,
    /// The path of the field within the data of the instruction, such as `config.bump`.
    pub path: Path,
}

/// Lists every struct field addressable within the data of the instruction at the end of
/// `instruction_path`, with its full path, following `definedTypeLinkNode`s, like
/// `getInstructionDataFields` in the JavaScript library.
///
/// Fields are only addressable where the data resolves to a struct, so nested fields are listed for
/// fields whose type is, or links to, a struct. Each defined type is followed at most once per branch,
/// which guards against cycles while still expanding sibling fields that link to the same type. The
/// path must contain the program of the instruction for linked data to be followed: otherwise, a
/// linked type contributes no fields. Fails with `UnexpectedNodeKind` when the path does not end with
/// an instruction.
///
/// ```
/// use codama_nodes::{
///     InstructionNode, IntegerTypeNode, NodeRef, StructFieldTypeNode, StructTypeNode, U16, U64,
/// };
/// use codama_visitors_core::{get_instruction_data_fields, LinkableDictionary, NodePath};
///
/// let instruction = InstructionNode {
///     identifier: "transfer".into(),
///     data: Box::new(Some(
///         StructTypeNode::new(vec![
///             StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
///             StructFieldTypeNode::new(
///                 "config",
///                 StructTypeNode::new(vec![StructFieldTypeNode::new("fee", IntegerTypeNode::le(U16))]),
///             ),
///         ])
///         .into(),
///     )),
///     ..InstructionNode::default()
/// };
/// let path = NodePath::from(vec![NodeRef::from(&instruction)]);
/// let fields = get_instruction_data_fields(&path, &LinkableDictionary::default()).unwrap();
/// let paths: Vec<_> = fields.iter().map(|field| field.path.to_string()).collect();
/// assert_eq!(paths, ["amount", "config", "config.fee"]);
/// ```
pub fn get_instruction_data_fields<'a>(
    instruction_path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> CodamaResult<Vec<InstructionDataField<'a>>> {
    let instruction = instruction_path.expect_last::<InstructionNode>()?;
    let mut walker = DataFieldsWalker {
        linkables,
        fields: vec![],
        followed_types: vec![],
    };
    if let Some(data) = instruction.data.as_ref() {
        let type_path = instruction_path.child(data);
        walker.walk(data, None, instruction_path, type_path);
    }
    Ok(walker.fields)
}

struct DataFieldsWalker<'l, 'a> {
    linkables: &'l LinkableDictionary<'a>,
    fields: Vec<InstructionDataField<'a>>,
    /// The defined types followed on the current branch, by address rather than identifier, as
    /// same-named types of different programs are distinct.
    followed_types: Vec<&'a DefinedTypeNode>,
}

impl<'a> DataFieldsWalker<'_, 'a> {
    /// Walks `r#type`, at the end of `type_path`. Links resolve from `context`, the path of the
    /// instruction or of the defined type that holds the type.
    fn walk(
        &mut self,
        r#type: &'a TypeNode,
        prefix: Option<&str>,
        context: &NodePath<'a>,
        type_path: NodePath<'a>,
    ) {
        match r#type {
            TypeNode::Link(link) => {
                let link_path = context.child(link);
                let Some((linked_path, defined_type)) =
                    self.linkables.get_path_of::<DefinedTypeNode>(&link_path)
                else {
                    return;
                };
                if self
                    .followed_types
                    .iter()
                    .any(|followed| std::ptr::eq(*followed, defined_type))
                {
                    return;
                }
                self.followed_types.push(defined_type);
                let type_path = linked_path.child(&*defined_type.r#type);
                self.walk(&defined_type.r#type, prefix, &linked_path, type_path);
                self.followed_types.pop();
            }
            TypeNode::Struct(r#struct) => {
                for field in &r#struct.fields {
                    let path = match prefix {
                        Some(prefix) => format!("{prefix}.{}", field.identifier),
                        None => field.identifier.to_string(),
                    };
                    let field_path = type_path.child(field);
                    self.fields.push(InstructionDataField {
                        field,
                        field_path: field_path.clone(),
                        path: path.parse().expect("identifiers joined by `.` form a path"),
                    });
                    if matches!(*field.r#type, TypeNode::Struct(_) | TypeNode::Link(_)) {
                        let mut nested_path = field_path;
                        nested_path.push(&*field.r#type);
                        self.walk(&field.r#type, Some(&path), context, nested_path);
                    }
                }
            }
            _ => {}
        }
    }
}
