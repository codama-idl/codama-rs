import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments, removeFromImportMap } from '@codama/fragments/rust';
import { isChildAttribute, type AttributeSpec, type NodeSpec, type Spec } from '@codama/spec';

import { getNodeMacroFlavor } from '../nodeFlavor';
import { getAttributeBodyLineFragment } from './attributeBodyLine';
import { use } from './helpers';

/**
 * Render the `#[<macro>] pub struct XxxNode { … }` declaration plus
 * any auto-derived `#[derive(...)]` line. The macro is one of
 * `#[node]` (default) or `#[type_node]` (for `typeNode` leaves of the
 * type category). Attributes are partitioned into "Data." and "Children."
 * sections; empty sections are omitted.
 *
 * Derive heuristic: `Default` when every field is unconditionally
 * Default-able (see {@link isUnconditionallyDefaultable}). Opaque
 * required fields disqualify the struct.
 */
export function getNodeStructFragment(node: NodeSpec, spec: Spec): Fragment {
    const structName = pascalCase(node.kind);
    const { data, children } = partitionAttributes(node);
    const flavor = getNodeMacroFlavor(node, spec);
    const macroName = flavor === 'type_node' ? 'codama_nodes_derive::type_node' : 'codama_nodes_derive::node';
    const macros = fragment`#[${use(macroName)}]`;
    const derives = buildDeriveFragment(node);
    const header = derives === undefined ? macros : mergeFragments([macros, derives], parts => parts.join('\n'));

    const raw =
        data.length === 0 && children.length === 0
            ? fragment`${header}\npub struct ${structName} {}`
            : fragment`${header}\npub struct ${structName} {\n${buildBody(data, children)}\n}`;

    // Drop the self-import a self-referential field (e.g.
    // `subInstructions: Vec<InstructionNode>`) would otherwise add —
    // the type is in scope via the local declaration.
    return {
        ...raw,
        imports: removeFromImportMap(raw.imports, `crate::${structName}`),
    };
}

function buildDeriveFragment(node: NodeSpec): Fragment | undefined {
    if (!node.attributes.every(isUnconditionallyDefaultable)) return undefined;
    return fragment`#[derive(Default)]`;
}

/**
 * `true` when this attribute's Rust type is guaranteed to implement
 * `Default` without introspecting any referenced type. Conservative:
 * "I'm not sure" => not Default-able, so the worst case is a missing
 * derive caught by `cargo build`, never uncompilable code.
 *
 * Sound shapes: any `optional` attribute, `array`, `docs`, scalar
 * primitives, the `String`-rendering kinds (`address`, `codamaVersion`,
 * `literal`), and `string` except the `integer`, `decimal` and `path`
 * constraints. Required `union`/`enumeration`/`node`/`literalUnion`/`text`,
 * `string:integer`, `string:decimal` and `string:path` reference opaque
 * types (`Text`, `Integer`, `Decimal`, `Path`) and disqualify the struct.
 */
function isUnconditionallyDefaultable(attr: AttributeSpec): boolean {
    if (attr.optional === true) return true;
    if (attr.type.kind === 'string') {
        const { constraint } = attr.type;
        return constraint !== 'integer' && constraint !== 'decimal' && constraint !== 'path';
    }
    const k = attr.type.kind;
    return (
        k === 'array' ||
        k === 'docs' ||
        k === 'boolean' ||
        k === 'integer' ||
        k === 'address' ||
        k === 'codamaVersion' ||
        k === 'literal'
    );
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
