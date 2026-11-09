import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { NodeSpec } from '@codama/spec';

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
 *   - Each node struct gets `TryFrom<Node>` and `From<&'a Struct> for NodeRef<'a>`.
 *     `From<Struct> for Node` comes from `derive_more::From`.
 *
 * Variant name = pascalCase(kind) minus the trailing `Node`
 * (`stringTypeNode` → `StringType`, `accountNode` → `Account`).
 */
export function getNodeEnumPageFragment(nodes: readonly NodeSpec[]): Fragment {
    const variants = nodes
        .map(node => ({ kind: node.kind, name: getNodeVariantName(node.kind), type: pascalCase(node.kind) }))
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

    const nodeRefImpl = fragment`impl NodeRef<'_> {
/// Clones the borrowed node into an owned \`Node\`.
pub fn to_node(self) -> Node {
match self {
${lines(v => fragment`Self::${v.name}(node) => Node::${v.name}(node.clone()),`)}
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
