import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { AttributeSpec, NodeSpec, Spec } from '@codama/spec';

import {
    type ChildTarget,
    getChildAttributes,
    getChildShape,
    getChildTarget,
    getUnionEnums,
    getVisitMethodName,
    getWalkerNodes,
    getWalkFunctionName,
    type UnionEnum,
} from '../nodeWalker';
import { getAttributeFieldShape, rustFieldName } from './attributeBodyLine';
import { formatDocs } from './enumPage';
import { nodeHasIdentifier } from './hasIdentifierImpl';
import { use } from './helpers';

/**
 * The body of `generated/node_visitor.rs`, in the style of `syn::visit`: the `NodeVisitor` trait, then one `walk_*`
 * function per node kind and per generated union enum. The external union enums have hand-written `walk_*`.
 */
export function getNodeVisitorPageFragment(spec: Spec): Fragment {
    const nodes = getWalkerNodes(spec);
    const unions = getUnionEnums(spec);
    const blocks = [
        getNodeVisitorTraitFragment(nodes, unions),
        ...nodes.map(node => getNodeWalkFragment(node, spec)),
        ...unions.map(getUnionWalkFragment),
    ];
    return mergeFragments(blocks, parts => parts.join('\n\n'));
}

function getNodeVisitorTraitFragment(nodes: readonly NodeSpec[], unions: readonly UnionEnum[]): Fragment {
    const docs = formatDocs([
        'A read-only walk over a node tree, with one `visit_*` method per node kind and per union enum.',
        'Each default `visit_*` method calls the `walk_*` function of the same name, which visits the children.',
        'An override that does not call its `walk_*` function skips the children of its node.',
        'A union method gets the path of the parent and the segment, as only the value knows the node to join.',
        'Start a walk with `visitor.visit_root(&root, &NodePath::default())`.',
    ]);
    const methods = [
        getEnterNodeMethodFragment(),
        ...nodes.map(getNodeVisitMethodFragment),
        ...unions.map(getUnionVisitMethodFragment),
    ];
    return fragment`${docs}pub trait NodeVisitor {\n${mergeFragments(methods, parts => parts.join('\n\n'))}\n}`;
}

function getEnterNodeMethodFragment(): Fragment {
    const docs = formatDocs(['Called by the `walk_*` function of each node kind, before it visits the children.']);
    const path = use('crate::NodePath');
    return fragment`${docs}fn enter_node<'a>(&mut self, _kind: &'static str, _path: &${path}<'a>) {}`;
}

/** One provided `visit_*` method of a node kind, which calls its `walk_*` function. */
export function getNodeVisitMethodFragment(node: NodeSpec): Fragment {
    const call = fragment`${getWalkFunctionName(node.kind)}(self, node, path)`;
    return getVisitMethodFragment(node.kind, getNodeParametersFragment(node.kind), call);
}

/** One provided `visit_*` method of a union enum. The `walk_*` of an external enum is imported from its module. */
export function getUnionVisitMethodFragment(union: UnionEnum): Fragment {
    const name = getWalkFunctionName(union.name);
    const walk = union.kind === 'external' ? use(`crate::${name}`) : fragment`${name}`;
    const call = fragment`${walk}(self, node, path, segment)`;
    return getVisitMethodFragment(union.name, getUnionParametersFragment(union.name), call);
}

function getVisitMethodFragment(name: string, parameters: Fragment, call: Fragment): Fragment {
    return fragment`fn ${getVisitMethodName(name)}<'a>(&mut self, ${parameters}) {\n${call};\n}`;
}

/** The `walk_*` function of a node kind: `enter_node`, then the children of each child attribute in order. */
export function getNodeWalkFragment(node: NodeSpec, spec: Spec): Fragment {
    const statements = [
        fragment`visitor.enter_node("${node.kind}", path);`,
        ...getChildAttributes(node).map(attribute => getChildVisitFragment(attribute, spec)),
    ];
    const body = mergeFragments(statements, parts => parts.join('\n'));
    return fragment`${getWalkSignatureFragment(node.kind, getNodeParametersFragment(node.kind))} {\n${body}\n}`;
}

/**
 * The statements that visit the children of one attribute.
 * The field is read with the wrapping of its struct field, so `Box` and `Option` stay transparent and an absent child
 * gives no visit. A `text` or `docs` value is visited only when it is a `TextNode`.
 */
export function getChildVisitFragment(attribute: AttributeSpec, spec: Spec): Fragment {
    const target = getChildTarget(attribute.type, spec);
    const field = `node.${rustFieldName(attribute.name)}`;
    if (getChildShape(attribute.type) === 'array') {
        const segment = fragment`${use('crate::PathSegment::Item')}("${attribute.name}", index)`;
        const visit = getVisitCallFragment(target, 'child', segment);
        return fragment`for (index, child) in ${field}.iter().enumerate() {\n${visit};\n}`;
    }

    const segment = fragment`${use('crate::PathSegment::Attribute')}("${attribute.name}")`;
    const pattern = getChildPatternFragment(attribute);
    if (pattern === undefined) return fragment`${getVisitCallFragment(target, `&${field}`, segment)};`;

    const value = derefsToOption(attribute) ? `&*${field}` : `&${field}`;
    const visit = getVisitCallFragment(target, 'child', segment);
    return fragment`if let ${pattern} = ${value} {\n${visit};\n}`;
}

/**
 * The pattern that binds `child` when the field may hold no child node, or `undefined` when it always holds one.
 * An optional field matches `Some(..)`, and a `text` or `docs` value matches only its `TextNode` variant.
 */
function getChildPatternFragment(attribute: AttributeSpec): Fragment | undefined {
    const isText = getChildShape(attribute.type) === 'text';
    const isOptional = ['option', 'boxedOption', 'docs'].includes(getAttributeFieldShape(attribute));
    if (!isText && !isOptional) return undefined;
    const binding = isText ? fragment`${use('crate::Text')}::Node(child)` : fragment`child`;
    return isOptional ? fragment`Some(${binding})` : binding;
}

/** A `Box<Option<T>>` or `Docs` field is read through `Deref` to reach its `Option`. */
function derefsToOption(attribute: AttributeSpec): boolean {
    const shape = getAttributeFieldShape(attribute);
    return shape === 'boxedOption' || shape === 'docs';
}

/**
 * The call that visits one child. A node child gets the path joined to it.
 * A union child gets the parent path and the segment, as only its value knows the node to join.
 */
function getVisitCallFragment(target: ChildTarget, value: string, segment: Fragment): Fragment {
    if (target.kind === 'union') {
        return fragment`visitor.${getVisitMethodName(target.name)}(${value}, path, ${segment})`;
    }
    const join = nodeHasIdentifier(target.node) ? 'join_named' : 'join';
    return fragment`visitor.${getVisitMethodName(target.node.kind)}(${value}, &path.${join}(${segment}, ${value}))`;
}

/** The `walk_*` function of a generated union enum, which visits the node inside the value. */
export function getUnionWalkFragment(union: UnionEnum): Fragment | undefined {
    if (union.kind === 'external') return undefined;
    const arms = union.variants.map(variant => {
        const visit = getVisitCallFragment({ kind: 'node', node: variant.node }, 'node', fragment`segment`);
        return fragment`${union.name}::${variant.name}(node) => ${visit},`;
    });
    const body = mergeFragments(arms, parts => parts.join('\n'));
    const signature = getWalkSignatureFragment(union.name, getUnionParametersFragment(union.name));
    return fragment`${signature} {\nmatch node {\n${body}\n}\n}`;
}

function getWalkSignatureFragment(name: string, parameters: Fragment): Fragment {
    return fragment`pub fn ${getWalkFunctionName(name)}<'a, V: NodeVisitor + ?Sized>(visitor: &mut V, ${parameters})`;
}

/** A node method gets the node and its own path. */
function getNodeParametersFragment(name: string): Fragment {
    return fragment`node: &'a ${use(`crate::${pascalCase(name)}`)}, path: &${use('crate::NodePath')}<'a>`;
}

/** A union method gets the parent path and the segment, since only the variant knows the node to join. */
function getUnionParametersFragment(name: string): Fragment {
    return fragment`${getNodeParametersFragment(name)}, segment: ${use('crate::PathSegment')}`;
}
