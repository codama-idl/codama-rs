import { pascalCase } from '@codama/fragments';
import { addFragmentImports, type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { NodeSpec } from '@codama/spec';

import { type ChildSlot, getChildSlots, getNodeFields } from '../childSlots';
import { use } from './helpers';

/**
 * The body for `node.rs`: the flat `Node` enum, its borrowed twin `NodeRef<'a>`,
 * and the conversions between them and every node struct.
 *
 *   - `Node` has one variant per node kind, sorted alphabetically, so it
 *     (de)serialises by `kind` through the `NodeUnion` derive.
 *   - Both enums are `#[non_exhaustive]`, so a new node kind in the spec
 *     is not a breaking change.
 *   - `NodeRef<'a>` has the same variants over `&'a` references, so a tree
 *     can be walked without cloning its children.
 *   - Each node struct gets `TryFrom<Node>`, `From<&'a Struct> for NodeRef<'a>` and
 *     `TryFrom<NodeRef<'a>> for &'a Struct`. `From<Struct> for Node` comes from `derive_more::From`.
 *   - `NodeRef` reads the child nodes of any node (`children`), rebuilds a node from
 *     its mapped children (`map_children`) and reads its identifier (`identifier`).
 *     A child is an attribute the spec marks with `isChildAttribute`, read in spec order.
 *
 * Variant name = pascalCase(kind) minus the trailing `Node`
 * (`stringTypeNode` → `StringType`, `accountNode` → `Account`).
 */
export function getNodeEnumPageFragment(nodes: readonly NodeSpec[]): Fragment {
    const variants = nodes
        .map(node => ({ kind: node.kind, name: getNodeVariantName(node.kind), node, type: pascalCase(node.kind) }))
        .toSorted((a, b) => a.name.localeCompare(b.name));

    const lines = (render: (v: (typeof variants)[number]) => Fragment) =>
        mergeFragments(variants.map(render), parts => parts.join('\n'));

    const nodeEnum = fragment`/// Any node of the Codama IDL, with one variant per node kind.
#[derive(${use('codama_nodes_derive::NodeUnion')}, ${use('derive_more::From')}, Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum Node {
${lines(v => fragment`${v.name}(${use(`crate::${v.type}`)}),`)}
}`;

    const nodeRefEnum = fragment`/// A borrowed node, with one variant per node kind, to walk a tree without cloning it.
#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
pub enum NodeRef<'a> {
${lines(v => fragment`${v.name}(&'a ${v.type}),`)}
}`;

    const nodeRefFromNode = fragment`impl<'a> From<&'a Node> for NodeRef<'a> {
fn from(node: &'a Node) -> Self {
match node {
${lines(v => fragment`Node::${v.name}(node) => Self::${v.name}(node),`)}
}
}
}`;

    const nodeRefImpl = fragment`impl<'a> NodeRef<'a> {
/// Clones the borrowed node into an owned \`Node\`.
pub fn to_node(self) -> Node {
match self {
${lines(v => fragment`Self::${v.name}(node) => Node::${v.name}(node.clone()),`)}
}
}

/// The identifier of the node, if it has one.
pub fn identifier(self) -> Option<&'a ${use('crate::Identifier')}> {
match self {
${mergeFragments(
    variants.flatMap(v => getIdentifierArmFragment(v.name, v.node) ?? []),
    parts => parts.join('\n'),
)}
_ => None,
}
}

/// The child nodes of the node, in the order of the spec attributes that hold them.
pub fn children(self) -> Vec<NodeRef<'a>> {
let mut children = Vec::new();
match self {
${lines(v => getChildrenArmFragment(v.name, v.node))}
}
children
}

/// Rebuilds the node from its children mapped by \`f\`, in the order of the spec attributes that hold them.
///
/// When \`f\` returns \`None\`, the child is deleted: an item is dropped from its list, an optional
/// child becomes \`None\`, and a required child deletes this node too, which returns \`None\`.
/// A mapped child of the wrong kind for its slot is an error.
// Data fields are cloned whatever their type, as the generator does not track which ones are \`Copy\`.
#[allow(clippy::clone_on_copy)]
pub fn map_children(self, mut f: impl FnMut(NodeRef<'a>) -> ${use('codama_errors::CodamaResult')}<Option<Node>>) -> CodamaResult<Option<Node>> {
let f: &mut ${use('crate::node_ref::MapChild')}<'_, 'a> = &mut f;
match self {
${lines(v => getMapChildrenArmFragment(v.name, v.type, v.node))}
}
}
}

impl ${use('crate::HasKind')} for NodeRef<'_> {
fn kind(&self) -> &'static str {
match self {
${lines(v => fragment`Self::${v.name}(node) => node.kind(),`)}
}
}
}`;

    const perNodeImpls = mergeFragments(
        variants.map(
            v => fragment`impl TryFrom<Node> for ${v.type} {
type Error = ${use('codama_errors::CodamaError')};

fn try_from(node: Node) -> Result<Self, Self::Error> {
match node {
Node::${v.name}(node) => Ok(node),
node => Err(CodamaError::InvalidNodeConversion {
from: node.kind().to_string(),
into: "${v.kind}".to_string(),
}),
}
}
}

impl<'a> From<&'a ${v.type}> for NodeRef<'a> {
fn from(node: &'a ${v.type}) -> Self {
Self::${v.name}(node)
}
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ${v.type} {
type Error = CodamaError;

fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
match node {
NodeRef::${v.name}(node) => Ok(node),
node => Err(CodamaError::InvalidNodeConversion {
from: node.kind().to_string(),
into: "${v.kind}".to_string(),
}),
}
}
}`,
        ),
        parts => parts.join('\n\n'),
    );

    return mergeFragments([nodeEnum, nodeRefEnum, nodeRefFromNode, nodeRefImpl, perNodeImpls], parts =>
        parts.join('\n\n'),
    );
}

/** `stringTypeNode` → `StringType`, `accountNode` → `Account`. */
export function getNodeVariantName(kind: string): string {
    const pascal = pascalCase(kind);
    return pascal.endsWith('Node') ? pascal.slice(0, -'Node'.length) : pascal;
}

function getIdentifierArmFragment(variant: string, node: NodeSpec): Fragment | undefined {
    const identifier = node.attributes.find(
        attr => attr.name === 'identifier' && attr.type.kind === 'string' && attr.type.constraint === 'identifier',
    );
    if (!identifier) return undefined;
    return identifier.optional === true
        ? fragment`Self::${variant}(node) => node.identifier.as_ref(),`
        : fragment`Self::${variant}(node) => Some(&node.identifier),`;
}

/** The match arm that pushes the children of one node kind onto `children`. */
function getChildrenArmFragment(variant: string, node: NodeSpec): Fragment {
    const slots = getChildSlots(node);
    const body = fragment`Self::${variant}(node) => {\n${slots.map(getChildReadLine).join('\n')}\n}`;
    const usesText = slots.some(slot => slot.kind === 'text' || slot.kind === 'docs');
    return usesText ? addFragmentImports(body, ['crate::Text']) : body;
}

/** The statement that pushes the children of one slot onto `children`. */
function getChildReadLine(slot: ChildSlot): string {
    const field = `node.${slot.field}`;
    switch (slot.kind) {
        case 'array':
            return `children.extend(${field}.iter().map(NodeRef::from));`;
        case 'docs':
            return `if let Some(${TEXT}::Node(child)) = &*${field} {\nchildren.push(NodeRef::from(child));\n}`;
        case 'text':
            return slot.optional
                ? `if let Some(${TEXT}::Node(child)) = &${field} {\nchildren.push(NodeRef::from(child));\n}`
                : `if let ${TEXT}::Node(child) = &${field} {\nchildren.push(NodeRef::from(child));\n}`;
        case 'node':
            return slot.optional
                ? `children.extend(${field}.as_ref().map(NodeRef::from));`
                : `children.push(NodeRef::from(&${field}));`;
        case 'union':
            return slot.optional
                ? `children.extend((*${field}).as_ref().map(NodeRef::from));`
                : `children.push(NodeRef::from(&*${field}));`;
        case 'anyNode':
            return `children.push(NodeRef::from(&*${field}));`;
    }
}

const TEXT = 'Text';

/**
 * The match arm that rebuilds one node kind from its mapped children.
 * The node is bound to `this`, since each mapped child goes into a local named after its field.
 */
function getMapChildrenArmFragment(variant: string, type: string, node: NodeSpec): Fragment {
    const statements: string[] = [];
    const fields: string[] = [];
    const imports = new Set<string>();
    for (const field of getNodeFields(node)) {
        if (field.kind === 'data') {
            fields.push(`${field.field}: this.${field.field}.clone()`);
            continue;
        }
        const { helper, statement, type, value } = getChildMapLine(field);
        if (helper) imports.add(`crate::node_ref::${helper}`);
        if (type) imports.add(`crate::${type}`);
        statements.push(statement);
        fields.push(value === field.field ? field.field : `${field.field}: ${value}`);
    }
    return addFragmentImports(arm(), [...imports]);

    function arm(): Fragment {
        return fragment`Self::${variant}(this) => {
${statements.join('\n')}
Ok(Some(Node::${variant}(${use(`crate::${type}`)} {
${fields.join(',\n')},
})))
}`;
    }
}

/**
 * The statement that maps one slot into a local of the same name, the value that fills the field,
 * and the helper and slot type that the statement refers to.
 */
function getChildMapLine(slot: ChildSlot): { helper?: string; statement: string; type?: string; value: string } {
    const local = slot.field;
    const field = `this.${slot.field}`;
    const required = (call: string) => `let Some(${local}) = ${call}? else {\nreturn Ok(None);\n};`;
    const optional = (call: string) => `let ${local} = ${call}?;`;
    switch (slot.kind) {
        case 'array':
            return {
                helper: 'map_array',
                statement: optional(`map_array::<${slot.type}, _>(&${field}, f)`),
                type: slot.type,
                value: local,
            };
        case 'docs':
            return { helper: 'map_docs', statement: optional(`map_docs(&${field}, f)`), value: local };
        case 'text':
            return slot.optional
                ? {
                      helper: 'map_optional_text',
                      statement: optional(`map_optional_text(${field}.as_ref(), f)`),
                      value: local,
                  }
                : { helper: 'map_text', statement: required(`map_text(&${field}, f)`), value: local };
        case 'node':
            return slot.optional
                ? {
                      helper: 'map_optional',
                      statement: optional(`map_optional::<${slot.type}>(${field}.as_ref().map(NodeRef::from), f)`),
                      type: slot.type,
                      value: local,
                  }
                : {
                      helper: 'map_required',
                      statement: required(`map_required::<${slot.type}>(NodeRef::from(&${field}), f)`),
                      type: slot.type,
                      value: local,
                  };
        case 'union':
            return slot.optional
                ? {
                      helper: 'map_optional',
                      statement: optional(`map_optional::<${slot.type}>((*${field}).as_ref().map(NodeRef::from), f)`),
                      type: slot.type,
                      value: `Box::new(${local})`,
                  }
                : {
                      helper: 'map_required',
                      statement: required(`map_required::<${slot.type}>(NodeRef::from(&*${field}), f)`),
                      type: slot.type,
                      value: `Box::new(${local})`,
                  };
        case 'anyNode':
            return { statement: required(`f(NodeRef::from(&*${field}))`), value: `Box::new(${local})` };
    }
}
