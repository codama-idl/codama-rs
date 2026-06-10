import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import { isChildAttribute, type AttributeSpec, type NodeSpec } from '@codama/spec';

import { getAttributeBodyLineFragment } from './attributeBodyLine';
import { use } from './helpers';

/**
 * Render the `#[node] pub struct XxxNode { … }` declaration for one
 * spec node. Attributes are partitioned into "Data." (primitives,
 * enumerations) and "Children." (node / union / nestedUnion refs,
 * recursively through `array` / `tuple`) sections; within each
 * section, fields appear in spec declaration order. Empty sections
 * are omitted.
 */
export function getNodeStructFragment(node: NodeSpec): Fragment {
    const structName = pascalCase(node.kind);
    const { data, children } = partitionAttributes(node);

    const body = buildBody(data, children);
    return fragment`#[${use('codama_nodes_derive::node')}]\npub struct ${structName} {\n${body}\n}`;
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

function buildBody(data: readonly AttributeSpec[], children: readonly AttributeSpec[]): Fragment {
    const sections: Fragment[] = [];
    if (data.length > 0) sections.push(buildSection('// Data.', data));
    if (children.length > 0) sections.push(buildSection('// Children.', children));
    return mergeFragments(sections, parts => parts.join('\n\n'));
}

function buildSection(header: string, attrs: readonly AttributeSpec[]): Fragment {
    const lines = attrs.map(getAttributeBodyLineFragment);
    const body = mergeFragments(lines, parts => parts.join('\n'));
    return fragment`${header}\n${body}`;
}
