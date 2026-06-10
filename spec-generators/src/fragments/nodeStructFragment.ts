import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments, removeFromImportMap } from '@codama/fragments/rust';
import { isChildAttribute, type AttributeSpec, type NodeSpec, type TypeExpr } from '@codama/spec';

import { FIELD_TYPE_OVERRIDES } from '../defaults';
import { getAttributeBodyLineFragment } from './attributeBodyLine';
import { use } from './helpers';

/**
 * Render the `#[node] pub struct XxxNode { … }` declaration plus any
 * auto-derived `#[derive(...)]` line. Attributes are partitioned into
 * "Data." (primitives, enumerations) and "Children." (node / union /
 * nestedUnion refs, recursively through array/tuple); empty sections
 * are omitted.
 *
 * Derive heuristic:
 *
 *   - `Copy`    when every attribute is a scalar kind (integer / float /
 *               boolean / enumeration), or the struct is empty.
 *   - `Default` when every field is unconditionally Default-able (see
 *               {@link isUnconditionallyDefaultable}). Opaque required
 *               fields (union / enumeration / node / nestedUnion /
 *               literalUnion, or overridden via FIELD_TYPE_OVERRIDES)
 *               disqualify the struct; the few such cases keep a
 *               hand-written `impl Default`.
 */
export function getNodeStructFragment(node: NodeSpec): Fragment {
    const structName = pascalCase(node.kind);
    const { data, children } = partitionAttributes(node);
    const macros = fragment`#[${use('codama_nodes_derive::node')}]`;
    const derives = buildDeriveFragment(node);
    const header = derives === undefined ? macros : mergeFragments([macros, derives], parts => parts.join('\n'));

    const raw =
        data.length === 0 && children.length === 0
            ? fragment`${header}\npub struct ${structName} {}`
            : fragment`${header}\npub struct ${structName} {\n${buildBody(node.kind, data, children)}\n}`;

    // Drop the self-import a self-referential field (e.g.
    // `subInstructions: Vec<InstructionNode>`) would otherwise add —
    // the type is in scope via the local declaration.
    return {
        ...raw,
        imports: removeFromImportMap(raw.imports, `crate::${structName}`),
    };
}

const SCALAR_KINDS: ReadonlySet<TypeExpr['kind']> = new Set(['integer', 'float', 'boolean', 'enumeration']);

function buildDeriveFragment(node: NodeSpec): Fragment | undefined {
    const isEmpty = node.attributes.length === 0;
    const isCopy = isEmpty || node.attributes.every(a => SCALAR_KINDS.has(a.type.kind));
    const isDefault = isEmpty || node.attributes.every(a => isUnconditionallyDefaultable(node.kind, a));
    const derives: string[] = [];
    if (isCopy) derives.push('Copy');
    if (isDefault) derives.push('Default');
    if (derives.length === 0) return undefined;
    return fragment`#[derive(${derives.join(', ')})]`;
}

/**
 * `true` when this attribute's Rust type is guaranteed to implement
 * `Default` without introspecting any referenced type. Conservative:
 * "I'm not sure" => not Default-able, so the worst case is a missing
 * derive caught by `cargo build`, never uncompilable code.
 *
 * Sound shapes: any `optional` attribute, `array`, `docs`, scalar
 * primitives, and the `String`-rendering kinds (`string`, `address`,
 * `codamaVersion`, `literal`). Required `union`/`enumeration`/`node`/
 * `nestedUnion`/`literalUnion` reference opaque types; required
 * `FIELD_TYPE_OVERRIDES` targets are also opaque.
 */
function isUnconditionallyDefaultable(nodeKind: string, attr: AttributeSpec): boolean {
    if (attr.optional === true) return true;
    if (FIELD_TYPE_OVERRIDES.has(`${nodeKind}.${attr.name}`)) return false;
    const k = attr.type.kind;
    return (
        k === 'array' ||
        k === 'docs' ||
        k === 'boolean' ||
        k === 'integer' ||
        k === 'float' ||
        k === 'string' ||
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
