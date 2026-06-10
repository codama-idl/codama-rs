import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import { isChildAttribute, type AttributeSpec, type NodeSpec, type TypeExpr } from '@codama/spec';

import { getAttributeBodyLineFragment } from './attributeBodyLine';
import { use } from './helpers';

/**
 * Render the `#[node] pub struct XxxNode { … }` declaration for one
 * spec node, plus any auto-derived `#[derive(...)]` line.
 *
 * Attributes are partitioned into "Data." (primitives, enumerations)
 * and "Children." (node / union / nestedUnion refs, recursively
 * through `array` / `tuple`) sections; within each section, fields
 * appear in spec declaration order. Empty sections are omitted.
 *
 * Derive heuristic (verified against every `#[node]` struct in
 * `codama-nodes`):
 *
 *   - `Copy`    when every attribute's type kind is a scalar
 *               (`integer` / `float` / `boolean` / `enumeration`);
 *               an empty struct also qualifies.
 *   - `Default` when the struct has no attributes at all.
 *
 * The handful of non-empty hand-written `Default` structs
 * (`ProgramNode`, `InstructionNode`, `InstructionStatusNode`) are
 * top-level builder types that keep their hand-written `Default`
 * impl and aren't generated here.
 */
export function getNodeStructFragment(node: NodeSpec): Fragment {
    const structName = pascalCase(node.kind);
    const { data, children } = partitionAttributes(node);
    const macros = fragment`#[${use('codama_nodes_derive::node')}]`;
    const derives = buildDeriveFragment(node);
    const header = derives === undefined ? macros : mergeFragments([macros, derives], parts => parts.join('\n'));

    if (data.length === 0 && children.length === 0) {
        return fragment`${header}\npub struct ${structName} {}`;
    }
    const body = buildBody(node.kind, data, children);
    return fragment`${header}\npub struct ${structName} {\n${body}\n}`;
}

const SCALAR_KINDS: ReadonlySet<TypeExpr['kind']> = new Set(['integer', 'float', 'boolean', 'enumeration']);

function buildDeriveFragment(node: NodeSpec): Fragment | undefined {
    const isEmpty = node.attributes.length === 0;
    const isCopy = isEmpty || node.attributes.every(a => SCALAR_KINDS.has(a.type.kind));
    const isDefault = isEmpty;
    const derives: string[] = [];
    if (isCopy) derives.push('Copy');
    if (isDefault) derives.push('Default');
    if (derives.length === 0) return undefined;
    return fragment`#[derive(${derives.join(', ')})]`;
}

interface PartitionedAttributes {
    readonly data: readonly AttributeSpec[];
    readonly children: readonly AttributeSpec[];
}

function partitionAttributes(node: NodeSpec): PartitionedAttributes {
    const data: AttributeSpec[] = [];
    const children: AttributeSpec[] = [];
    for (const attr of node.attributes) {
        if (isChildAttribute(attr.type)) children.push(attr);
        else data.push(attr);
    }
    return { data, children };
}

function buildBody(nodeKind: string, data: readonly AttributeSpec[], children: readonly AttributeSpec[]): Fragment {
    const sections: Fragment[] = [];
    if (data.length > 0) sections.push(buildSection(nodeKind, '// Data.', data));
    if (children.length > 0) sections.push(buildSection(nodeKind, '// Children.', children));
    return mergeFragments(sections, parts => parts.join('\n\n'));
}

function buildSection(nodeKind: string, header: string, attrs: readonly AttributeSpec[]): Fragment {
    const lines = attrs.map(attr => getAttributeBodyLineFragment(nodeKind, attr));
    const body = mergeFragments(lines, parts => parts.join('\n'));
    return fragment`${header}\n${body}`;
}
