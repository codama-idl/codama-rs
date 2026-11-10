use crate::{visit, IdentityVisitor, NodePath, VisitorExt};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{
    HasKind, Identifier, InjectedValueNode, Node, NodeRef, ProvidedNode, ValueNode,
};
use std::cell::{Cell, RefCell};

/// The `providedNode`s visible at a point of the tree, to resolve the `injectedValueNode`s found there.
///
/// Each `instructionNode` with `provides` adds a frame, and
/// [`VisitorExt::record_provided_scope`](crate::VisitorExt::record_provided_scope) keeps the frames in
/// sync with a visit. Resolution is lexical: the innermost frame providing a key wins, and outer
/// frames stay visible, so a sub-instruction can shadow a key that its parent provides. Frames are
/// independent of the [`NodeStack`](crate::NodeStack), so following a link, for example into a defined
/// type, keeps the frames of the instruction that uses it.
///
/// Its methods take `&self`, so visitors and their extensions can share it by reference.
///
/// ```
/// use codama_nodes::{InjectedValueNode, IntegerValueNode, ProvidedNode, ValueNode};
/// use codama_visitors_core::ProvidedScope;
///
/// let provides = vec![ProvidedNode::new("decimals", IntegerValueNode::new(6))];
/// let scope = ProvidedScope::new([provides.as_slice()]);
///
/// let injected = InjectedValueNode::new("decimals");
/// let resolved = scope.resolve::<ValueNode>(&injected).unwrap();
/// assert_eq!(resolved, Some(IntegerValueNode::new(6).into()));
/// ```
#[derive(Debug, Clone, Default)]
pub struct ProvidedScope<'a> {
    /// The `provides` of each frame, from the outermost to the innermost.
    frames: RefCell<Vec<&'a [ProvidedNode]>>,
}

/// A node that an injection resolved to, with the depth of the frames its own injections resolve against.
struct Resolution<'a> {
    depth: usize,
    node: NodeRef<'a>,
    provider: Option<&'a ProvidedNode>,
}

/// The result of replacing the injections within a node.
enum Within<'a> {
    /// The node has no injection.
    Unchanged,
    /// The node with every injection replaced, boxed as a node is much larger than the other variants.
    Replaced(Box<Node>),
    /// The first injection that resolves to nothing.
    Missing(&'a InjectedValueNode),
}

impl<'a> ProvidedScope<'a> {
    /// A scope with the given frames, from the outermost to the innermost.
    pub fn new(frames: impl IntoIterator<Item = &'a [ProvidedNode]>) -> Self {
        Self {
            frames: RefCell::new(frames.into_iter().collect()),
        }
    }

    /// Opens a new innermost frame with the given providers.
    pub fn push(&self, provides: &'a [ProvidedNode]) {
        self.frames.borrow_mut().push(provides);
    }

    /// Closes the innermost frame.
    pub fn pop(&self) {
        self.frames.borrow_mut().pop();
    }

    /// The number of open frames.
    ///
    /// [`get_provided_scope_from_path`] opens a frame for every instruction of the path, like in the
    /// JavaScript library, while [`VisitorExt::record_provided_scope`](crate::VisitorExt::record_provided_scope)
    /// skips the instructions without `provides`. Empty frames never change a resolution.
    pub fn depth(&self) -> usize {
        self.frames.borrow().len()
    }

    /// The innermost provider of `key`, if any.
    pub fn get(&self, key: &Identifier) -> Option<&'a ProvidedNode> {
        let depth = self.frames.borrow().len();
        self.find(key, depth).map(|(_, provider)| provider)
    }

    /// Resolves every `injectedValueNode` within `node`, itself included, into a `T` without injections.
    ///
    /// An injection resolves to the innermost provided node for its key. A provided node resolves
    /// against the frames outside the one providing it, so a provider can inject its own key. The
    /// `fallback` of an injection is used when no frame provides its key, or when the chain of
    /// providers dead-ends. Resolution is all or nothing: if any injection within `node` resolves to
    /// nothing, this returns `None` rather than a partially resolved value.
    ///
    /// The node that `node` resolves to must convert into `T`, such as `ValueNode`. When it comes from
    /// a provider, the error is `InvalidProvidedValue` and names that provider. The injections nested
    /// within it must fit the attributes that hold them.
    pub fn resolve<'n, T>(&self, node: impl Into<NodeRef<'n>>) -> CodamaResult<Option<T>>
    where
        'a: 'n,
        T: TryFrom<Node, Error = CodamaError>,
    {
        Ok(self.resolve_outcome(node.into())?.ok())
    }

    /// Like [`ProvidedScope::resolve`], with an `InjectedValueNotProvided` error naming the first
    /// injection that resolves to nothing.
    ///
    /// Use it where the value is required, such as to encode a constant. Where an absent value is
    /// acceptable, such as an injection within a defined type used outside of any instruction, use
    /// [`ProvidedScope::resolve`].
    pub fn resolve_or_err<'n, T>(&self, node: impl Into<NodeRef<'n>>) -> CodamaResult<T>
    where
        'a: 'n,
        T: TryFrom<Node, Error = CodamaError>,
    {
        self.resolve_outcome(node.into())?.map_err(|missing| {
            CodamaError::InjectedValueNotProvided {
                key: missing.key.to_string(),
                path: NodePath::from(vec![NodeRef::from(missing)]).to_string(),
            }
        })
    }

    fn resolve_outcome<'n, T>(
        &self,
        node: NodeRef<'n>,
    ) -> CodamaResult<Result<T, &'n InjectedValueNode>>
    where
        'a: 'n,
        T: TryFrom<Node, Error = CodamaError>,
    {
        let depth = self.frames.borrow().len();
        let resolution = match node {
            NodeRef::InjectedValue(injected) => match self.lookup(injected, depth) {
                Some(resolution) => resolution,
                None => return Ok(Err(injected)),
            },
            node => Resolution {
                depth,
                node,
                provider: None,
            },
        };

        // The resolved node must convert into `T` before its nested injections are resolved.
        let root = match (T::try_from(resolution.node.to_node()), resolution.provider) {
            (Ok(root), _) => root,
            (Err(CodamaError::InvalidNodeConversion { into, .. }), Some(provider)) => {
                // The path ends with the provided value, unless the provider provides an injection,
                // whose fallback the value is, as a resolved node is never an injection.
                let mut path = vec![NodeRef::from(provider)];
                if !matches!(&*provider.node, Node::InjectedValue(_)) {
                    path.push(resolution.node);
                }
                return Err(CodamaError::InvalidProvidedValue {
                    expected: into,
                    path: NodePath::from(path).to_string(),
                });
            }
            (Err(error), _) => return Err(error),
        };

        match self.resolve_within(resolution.node, resolution.depth)? {
            Within::Unchanged => Ok(Ok(root)),
            Within::Replaced(node) => T::try_from(*node).map(Ok),
            Within::Missing(missing) => Ok(Err(missing)),
        }
    }

    /// Finds the node that `node` stands for, searching the frames below `depth`.
    fn lookup<'n>(&self, node: &'n InjectedValueNode, depth: usize) -> Option<Resolution<'n>>
    where
        'a: 'n,
    {
        if let Some((provider_depth, provider)) = self.find(&node.key, depth) {
            match &*provider.node {
                Node::InjectedValue(injected) => {
                    if let Some(chained) = self.lookup(injected, provider_depth) {
                        return Some(Resolution {
                            provider: chained.provider.or(Some(provider)),
                            ..chained
                        });
                    }
                }
                provided => {
                    return Some(Resolution {
                        depth: provider_depth,
                        node: NodeRef::from(provided),
                        provider: Some(provider),
                    })
                }
            }
        }
        match &*node.fallback {
            None => None,
            Some(ValueNode::Injected(fallback)) => self.lookup(fallback, depth),
            Some(fallback) => Some(Resolution {
                depth,
                node: NodeRef::from(fallback),
                provider: None,
            }),
        }
    }

    /// Replaces every injection within `node`, resolving them against the frames below `depth`.
    ///
    /// Each injection resolves as a whole, from the top of the tree down, so its `fallback` is only
    /// resolved when its own key does not resolve.
    fn resolve_within<'n>(&self, node: NodeRef<'n>, depth: usize) -> CodamaResult<Within<'n>>
    where
        'a: 'n,
    {
        let replaced = Cell::new(false);
        let missing = Cell::new(None);
        let visitor = IdentityVisitor.extend(|child, scope| {
            if missing.get().is_some() {
                return Ok(Some(child.to_node()));
            }
            let NodeRef::InjectedValue(injected) = child else {
                return scope.next(child);
            };
            replaced.set(true);
            let Some(resolution) = self.lookup(injected, depth) else {
                missing.set(Some(injected));
                return Ok(None);
            };
            match self.resolve_within(resolution.node, resolution.depth)? {
                Within::Unchanged => Ok(Some(resolution.node.to_node())),
                Within::Replaced(node) => Ok(Some(*node)),
                Within::Missing(inner) => {
                    missing.set(Some(inner));
                    Ok(None)
                }
            }
        });
        let result = visit(&visitor, node)?;
        if let Some(missing) = missing.get() {
            return Ok(Within::Missing(missing));
        }
        if !replaced.get() {
            return Ok(Within::Unchanged);
        }
        result
            .map(|node| Within::Replaced(Box::new(node)))
            .ok_or_else(|| CodamaError::DeletedNode {
                kind: node.kind().to_string(),
            })
    }

    /// The innermost provider of `key` in the frames below `depth`, with the depth of its frame.
    fn find(&self, key: &Identifier, depth: usize) -> Option<(usize, &'a ProvidedNode)> {
        let frames = self.frames.borrow();
        frames[..depth]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, frame)| {
                frame
                    .iter()
                    .find(|provider| provider.identifier == *key)
                    .map(|provider| (index, provider))
            })
    }
}

/// The scope of the `provides` of every instruction of `path`, from the outermost to the innermost,
/// so the injections within the last node of `path` resolve from the instructions that enclose it.
pub fn get_provided_scope_from_path<'a>(path: &NodePath<'a>) -> ProvidedScope<'a> {
    ProvidedScope::new(path.nodes().iter().filter_map(|node| match node {
        NodeRef::Instruction(instruction) => Some(instruction.provides.as_slice()),
        _ => None,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        AccountValueNode, InstructionInputValueNode, InstructionNode, IntegerValueNode,
        PdaSeedValueNode, ProgramNode, PublicKeyTypeNode, RootNode, StringValueNode,
        StructFieldValueNode, StructValueNode,
    };

    /// The kinds that most tests expect, like `['accountValueNode', 'integerValueNode']` in the
    /// JavaScript tests.
    type Input = InstructionInputValueNode;

    fn provide(identifier: &'static str, node: impl Into<Node>) -> ProvidedNode {
        ProvidedNode::new(identifier, node)
    }

    fn inject(key: &'static str) -> InjectedValueNode {
        InjectedValueNode::new(key)
    }

    fn inject_or(key: &'static str, fallback: impl Into<ValueNode>) -> InjectedValueNode {
        InjectedValueNode::with_fallback(key, fallback)
    }

    fn person(age: impl Into<ValueNode>) -> StructValueNode {
        StructValueNode::new(vec![
            StructFieldValueNode::new("name", StringValueNode::new("Alice")),
            StructFieldValueNode::new("age", age),
        ])
    }

    #[test]
    fn resolves_an_injection_to_its_provided_node() {
        let frame = [provide("decimals", IntegerValueNode::new(6))];
        let scope = ProvidedScope::new([frame.as_slice()]);
        assert_eq!(
            scope.resolve::<Input>(&inject("decimals")).unwrap(),
            Some(IntegerValueNode::new(6).into())
        );
    }

    #[test]
    fn lets_inner_frames_shadow_outer_frames() {
        let outer = [provide("authority", AccountValueNode::new("outer"))];
        let inner = [provide("authority", AccountValueNode::new("inner"))];
        let scope = ProvidedScope::new([outer.as_slice(), inner.as_slice()]);
        assert_eq!(
            scope.resolve::<Input>(&inject("authority")).unwrap(),
            Some(AccountValueNode::new("inner").into())
        );
    }

    #[test]
    fn keeps_outer_frames_visible() {
        let outer = [provide("decimals", IntegerValueNode::new(6))];
        let inner = [provide("authority", AccountValueNode::new("owner"))];
        let scope = ProvidedScope::new([outer.as_slice(), inner.as_slice()]);
        assert_eq!(
            scope.resolve::<Input>(&inject("decimals")).unwrap(),
            Some(IntegerValueNode::new(6).into())
        );
    }

    #[test]
    fn returns_the_fallback_when_no_frame_provides_the_key() {
        let node = inject_or("decimals", IntegerValueNode::new(9));
        assert_eq!(
            ProvidedScope::default().resolve::<Input>(&node).unwrap(),
            Some(IntegerValueNode::new(9).into())
        );
    }

    #[test]
    fn returns_none_when_no_frame_provides_the_key_and_there_is_no_fallback() {
        assert_eq!(
            ProvidedScope::default()
                .resolve::<Input>(&inject("decimals"))
                .unwrap(),
            None
        );
    }

    #[test]
    fn resolves_an_injected_fallback_against_the_same_frames() {
        let frame = [provide("defaultDecimals", IntegerValueNode::new(9))];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let node = inject_or("decimals", inject("defaultDecimals"));
        assert_eq!(
            scope.resolve::<Input>(&node).unwrap(),
            Some(IntegerValueNode::new(9).into())
        );
    }

    #[test]
    fn resolves_a_provided_injection_against_the_outer_frames_only() {
        // An inner frame re-provides a key by injecting that same key, which resolves to the outer
        // provider rather than looping.
        let outer = [provide("decimals", IntegerValueNode::new(6))];
        let inner = [provide("decimals", inject("decimals"))];
        let scope = ProvidedScope::new([outer.as_slice(), inner.as_slice()]);
        assert_eq!(
            scope.resolve::<Input>(&inject("decimals")).unwrap(),
            Some(IntegerValueNode::new(6).into())
        );
    }

    #[test]
    fn uses_the_fallback_when_a_provided_injection_chain_dead_ends() {
        let frame = [provide("decimals", inject("missing"))];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let node = inject_or("decimals", IntegerValueNode::new(9));
        assert_eq!(
            scope.resolve::<Input>(&node).unwrap(),
            Some(IntegerValueNode::new(9).into())
        );
    }

    #[test]
    fn resolves_injections_nested_within_a_node() {
        let frame = [provide("age", IntegerValueNode::new(42))];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let node = person(inject("age"));
        assert_eq!(
            scope.resolve::<ValueNode>(&node).unwrap(),
            Some(person(IntegerValueNode::new(42)).into())
        );
    }

    #[test]
    fn treats_a_node_with_an_unresolvable_nested_injection_as_unresolvable() {
        let node = person(inject("age"));
        assert_eq!(
            ProvidedScope::default()
                .resolve::<ValueNode>(&node)
                .unwrap(),
            None
        );
    }

    #[test]
    fn does_not_require_the_fallback_of_a_nested_injection_whose_key_is_provided() {
        let frame = [provide("authority", AccountValueNode::new("owner"))];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let node = PdaSeedValueNode::new("authority", inject_or("authority", inject("payer")));
        assert_eq!(
            scope.resolve::<PdaSeedValueNode>(&node).unwrap(),
            Some(PdaSeedValueNode::new(
                "authority",
                AccountValueNode::new("owner")
            ))
        );
    }

    #[test]
    fn returns_a_node_without_injections_as_is() {
        let node = StructValueNode::new(vec![StructFieldValueNode::new(
            "name",
            StringValueNode::new("Alice"),
        )]);
        assert_eq!(
            ProvidedScope::default()
                .resolve::<ValueNode>(&node)
                .unwrap(),
            Some(node.into())
        );
    }

    #[test]
    fn attributes_a_wrong_kind_fallback_of_a_provided_injection_to_its_provider() {
        // The provider provides an injection whose own fallback has the wrong kind, so the error names
        // the provider but not the injection it provides, which the fallback replaces.
        let frame = [provide(
            "symbol",
            inject_or("missing", IntegerValueNode::new(9)),
        )];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let error = scope
            .resolve::<StringValueNode>(&inject("symbol"))
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "The value provided at [providedNode]symbol is not one of [stringValueNode]."
        );
    }

    #[test]
    fn resolves_injections_within_a_provided_node_against_the_outer_frames() {
        let outer = [provide("age", IntegerValueNode::new(1))];
        let inner = [
            provide("age", IntegerValueNode::new(2)),
            provide(
                "person",
                StructValueNode::new(vec![StructFieldValueNode::new("age", inject("age"))]),
            ),
        ];
        let scope = ProvidedScope::new([outer.as_slice(), inner.as_slice()]);
        assert_eq!(
            scope.resolve::<ValueNode>(&inject("person")).unwrap(),
            Some(
                StructValueNode::new(vec![StructFieldValueNode::new(
                    "age",
                    IntegerValueNode::new(1)
                )])
                .into()
            )
        );
    }

    #[test]
    fn fails_when_a_fallback_is_not_of_the_expected_kind() {
        let node = inject_or("symbol", IntegerValueNode::new(9));
        let error = ProvidedScope::default()
            .resolve::<StringValueNode>(&node)
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "Could not convert node from `integerValueNode` into `stringValueNode`"
        );
    }

    #[test]
    fn fails_when_the_provided_node_is_not_of_the_expected_kind() {
        let frame = [provide("decimals", PublicKeyTypeNode::new())];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let error = scope.resolve::<Input>(&inject("decimals")).unwrap_err();
        assert_eq!(
            error.to_string(),
            "The value provided at [providedNode]decimals.[publicKeyTypeNode] is not one of [InstructionInputValueNode]."
        );
    }

    #[test]
    fn resolve_or_err_resolves_an_injection_to_its_provided_node() {
        let frame = [provide("decimals", IntegerValueNode::new(6))];
        let scope = ProvidedScope::new([frame.as_slice()]);
        assert_eq!(
            scope.resolve_or_err::<Input>(&inject("decimals")).unwrap(),
            IntegerValueNode::new(6).into()
        );
    }

    #[test]
    fn resolve_or_err_returns_the_fallback_when_no_frame_provides_the_key() {
        let node = inject_or("decimals", IntegerValueNode::new(9));
        assert_eq!(
            ProvidedScope::default()
                .resolve_or_err::<Input>(&node)
                .unwrap(),
            IntegerValueNode::new(9).into()
        );
    }

    fn not_provided(key: &str) -> String {
        format!("Injected value [{key}] is not provided and has no fallback at [injectedValueNode]. Values injected into a defined type are provided by the instruction that uses it.")
    }

    #[test]
    fn resolve_or_err_fails_when_no_frame_provides_the_key_and_there_is_no_fallback() {
        let error = ProvidedScope::default()
            .resolve_or_err::<Input>(&inject("decimals"))
            .unwrap_err();
        assert_eq!(error.to_string(), not_provided("decimals"));
    }

    #[test]
    fn resolve_or_err_names_the_nested_injection_that_resolves_to_nothing() {
        let error = ProvidedScope::default()
            .resolve_or_err::<ValueNode>(&person(inject("age")))
            .unwrap_err();
        assert_eq!(error.to_string(), not_provided("age"));
    }

    #[test]
    fn resolve_or_err_names_the_injection_a_provided_chain_dead_ends_on() {
        let frame = [provide(
            "person",
            StructValueNode::new(vec![StructFieldValueNode::new("age", inject("missing"))]),
        )];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let error = scope
            .resolve_or_err::<ValueNode>(&inject("person"))
            .unwrap_err();
        assert_eq!(error.to_string(), not_provided("missing"));
    }

    #[test]
    fn resolve_or_err_fails_when_the_provided_node_is_not_of_the_expected_kind() {
        let frame = [provide("decimals", PublicKeyTypeNode::new())];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let error = scope
            .resolve_or_err::<Input>(&inject("decimals"))
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "The value provided at [providedNode]decimals.[publicKeyTypeNode] is not one of [InstructionInputValueNode]."
        );
    }

    #[test]
    fn pushes_and_pops_frames() {
        let frame = [provide("symbol", StringValueNode::new("SOL"))];
        let scope = ProvidedScope::default();
        scope.push(&frame);
        assert_eq!(scope.get(&Identifier::from("symbol")), Some(&frame[0]));
        assert_eq!(scope.depth(), 1);
        scope.pop();
        assert_eq!(scope.get(&Identifier::from("symbol")), None);
        assert_eq!(scope.depth(), 0);
    }

    #[test]
    fn clones_into_an_independent_scope() {
        let frame = [provide("symbol", StringValueNode::new("SOL"))];
        let scope = ProvidedScope::new([frame.as_slice()]);
        let clone = scope.clone();
        clone.pop();
        assert!(scope.get(&Identifier::from("symbol")).is_some());
        assert!(clone.get(&Identifier::from("symbol")).is_none());
    }

    #[test]
    fn opens_a_frame_for_every_instruction_of_a_path_outermost_first() {
        // A sub-instruction shadows a key provided by its parent.
        let child = InstructionNode {
            identifier: "child".into(),
            provides: vec![provide("fee", IntegerValueNode::new(2))],
            ..InstructionNode::default()
        };
        let parent = InstructionNode {
            identifier: "parent".into(),
            provides: vec![
                provide("fee", IntegerValueNode::new(1)),
                provide("decimals", IntegerValueNode::new(6)),
            ],
            sub_instructions: vec![child],
            ..InstructionNode::default()
        };
        let root = RootNode::new(ProgramNode {
            instructions: vec![parent],
            ..ProgramNode::new("myProgram", "1111")
        });
        let parent = &root.program.instructions[0];
        let path = NodePath::from(vec![
            NodeRef::from(&root),
            NodeRef::from(&root.program),
            NodeRef::from(parent),
            NodeRef::from(&parent.sub_instructions[0]),
        ]);
        let scope = get_provided_scope_from_path(&path);
        assert_eq!(
            scope.resolve::<Input>(&inject("fee")).unwrap(),
            Some(IntegerValueNode::new(2).into())
        );
        assert_eq!(
            scope.resolve::<Input>(&inject("decimals")).unwrap(),
            Some(IntegerValueNode::new(6).into())
        );
    }
}
