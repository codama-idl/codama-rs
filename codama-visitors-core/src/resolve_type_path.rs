use crate::{LinkableDictionary, NodePath};
use codama_errors::CodamaError;
use codama_nodes::{AccountNode, DefinedTypeNode, NodeRef, Path, PathSegment};
use std::fmt;

/// Options of [`resolve_type_path`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ResolveTypePathOptions {
    /// Whether a `.data` segment applied to a public key whose type carries an `account_link` continues
    /// into the data of the linked account, such as `mint.data.decimals`. Off by default, as following
    /// such paths requires reading account state: field discriminators, for instance, must not cross
    /// account links.
    pub follow_account_links: bool,
}

/// Why [`resolve_type_path`] could not resolve a path. It converts into a [`CodamaError`], so `?` works
/// in functions returning a `CodamaResult`.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolveTypePathError<'a> {
    /// The `segment` of `path_string` does not apply to the node at the end of `path`.
    CannotResolvePath {
        path: NodePath<'a>,
        path_string: Path,
        segment: String,
    },
    /// The link at the end of `path` points to no recorded node.
    LinkedNodeNotFound { path: NodePath<'a> },
}

impl fmt::Display for ResolveTypePathError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        CodamaError::from(self.clone()).fmt(f)
    }
}

impl From<ResolveTypePathError<'_>> for CodamaError {
    fn from(error: ResolveTypePathError<'_>) -> Self {
        match error {
            ResolveTypePathError::CannotResolvePath {
                path,
                path_string,
                segment,
            } => CodamaError::CannotResolvePath {
                segment,
                path_string: path_string.to_string(),
                path: path.to_string(),
            },
            ResolveTypePathError::LinkedNodeNotFound { path } => CodamaError::LinkedNodeNotFound {
                path: path.to_string(),
            },
        }
    }
}

/// Resolves `path` against the type at the end of `source` and returns the full path of the node it
/// points to, like `resolveTypePath` in the JavaScript library.
///
/// A field segment such as `.config` selects a struct field, and an index segment such as `[0]`
/// selects the n-th item of a tuple, or the item of an array or a set. Struct fields, enum variants
/// and `definedTypeLinkNode`s along the way resolve to their types, following links to their
/// definitions, so the returned path reflects where each node is defined. The last node is returned as
/// addressed, such as a struct field whose type is a link. Indices into arrays and sets are not
/// checked against their count, as they select the item type.
///
/// With [`ResolveTypePathOptions::follow_account_links`], a `.data` segment applied to a public key with
/// an `account_link` resolves to the data of the linked account: `mint.data.decimals` reads the
/// `decimals` of the account that the `mint` field links to, and `mint.data` its whole data. Other
/// segments applied to such a public key are reserved for other attributes of the account, so they
/// fail.
///
/// ```
/// use codama_nodes::{
///     InstructionNode, IntegerTypeNode, NodeRef, ProgramNode, RootNode, StructFieldTypeNode,
///     StructTypeNode, U64,
/// };
/// use codama_visitors_core::{
///     get_linkables_from_node, resolve_type_path, NodePath, ResolveTypePathOptions,
/// };
///
/// let root = RootNode::new(ProgramNode {
///     instructions: vec![InstructionNode {
///         identifier: "transfer".into(),
///         data: Box::new(Some(
///             StructTypeNode::new(vec![StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64))])
///                 .into(),
///         )),
///         ..InstructionNode::default()
///     }],
///     ..ProgramNode::new("token", "1111")
/// });
/// let instruction = &root.program.instructions[0];
/// let data = instruction.data.as_ref().as_ref().unwrap();
/// let source = NodePath::from(vec![
///     NodeRef::from(&root),
///     NodeRef::from(&root.program),
///     NodeRef::from(instruction),
///     NodeRef::from(data),
/// ]);
///
/// let linkables = get_linkables_from_node(&root);
/// let path = "amount".parse().unwrap();
/// let resolved = resolve_type_path(&source, &path, &linkables, ResolveTypePathOptions::default());
/// assert_eq!(
///     resolved.unwrap().to_string(),
///     "[programNode]token.[instructionNode]transfer.[structTypeNode].[structFieldTypeNode]amount"
/// );
/// ```
pub fn resolve_type_path<'a>(
    source: &NodePath<'a>,
    path: &Path,
    linkables: &LinkableDictionary<'a>,
    options: ResolveTypePathOptions,
) -> Result<NodePath<'a>, ResolveTypePathError<'a>> {
    let mut current = source.clone();
    for segment in path.segments() {
        let mut container = resolve_container(current, path, segment, linkables)?;
        let node = container.last().expect("a type path is never empty");
        match (segment, node) {
            (PathSegment::Field("data"), NodeRef::PublicKeyType(public_key))
                if options.follow_account_links && public_key.account_link.is_some() =>
            {
                let link = public_key.account_link.as_ref().expect("checked above");
                container.push(link);
                let mut account_path = linkables
                    .get_path(&container)
                    .ok_or(ResolveTypePathError::LinkedNodeNotFound { path: container })?;
                let account: &AccountNode = account_path
                    .last()
                    .and_then(|node| node.try_into().ok())
                    .expect("an account link resolves to an account");
                account_path.push(&*account.data);
                current = account_path;
            }
            (PathSegment::Field(identifier), NodeRef::StructType(r#struct)) => {
                match r#struct
                    .fields
                    .iter()
                    .find(|field| field.identifier.as_str() == identifier)
                {
                    Some(field) => {
                        container.push(field);
                        current = container;
                    }
                    None => return Err(cannot_resolve(container, path, segment)),
                }
            }
            (PathSegment::Index(index), NodeRef::TupleType(tuple)) => {
                match tuple.items.get(index) {
                    Some(item) => {
                        container.push(item);
                        current = container;
                    }
                    None => return Err(cannot_resolve(container, path, segment)),
                }
            }
            (PathSegment::Index(_), NodeRef::ArrayType(array)) => {
                container.push(&*array.item);
                current = container;
            }
            (PathSegment::Index(_), NodeRef::SetType(set)) => {
                container.push(&*set.item);
                current = container;
            }
            _ => return Err(cannot_resolve(container, path, segment)),
        }
    }
    Ok(current)
}

/// Resolves struct fields, enum variants and defined type links into the type they describe, so the
/// next segment can apply to it.
fn resolve_container<'a>(
    mut current: NodePath<'a>,
    path: &Path,
    segment: PathSegment,
    linkables: &LinkableDictionary<'a>,
) -> Result<NodePath<'a>, ResolveTypePathError<'a>> {
    let mut followed_types: Vec<*const DefinedTypeNode> = vec![];
    loop {
        match current.last().expect("a type path is never empty") {
            NodeRef::StructFieldType(field) => current.push(&*field.r#type),
            NodeRef::EnumVariantType(variant) => match variant.data.as_ref() {
                Some(data) => current.push(data),
                None => return Err(cannot_resolve(current, path, segment)),
            },
            NodeRef::DefinedTypeLink(_) => {
                let Some(mut defined_type_path) = linkables.get_path(&current) else {
                    return Err(ResolveTypePathError::LinkedNodeNotFound { path: current });
                };
                let defined_type: &DefinedTypeNode = defined_type_path
                    .last()
                    .and_then(|node| node.try_into().ok())
                    .expect("a defined type link resolves to a defined type");
                // An alias cycle, such as `a = b` and `b = a`, never reaches a type to apply the segment to.
                if followed_types.contains(&(defined_type as *const _)) {
                    return Err(cannot_resolve(current, path, segment));
                }
                followed_types.push(defined_type);
                defined_type_path.push(&*defined_type.r#type);
                current = defined_type_path;
            }
            _ => return Ok(current),
        }
    }
}

fn cannot_resolve<'a>(
    path: NodePath<'a>,
    path_string: &Path,
    segment: PathSegment,
) -> ResolveTypePathError<'a> {
    ResolveTypePathError::CannotResolvePath {
        path,
        path_string: path_string.clone(),
        segment: segment.to_string(),
    }
}
