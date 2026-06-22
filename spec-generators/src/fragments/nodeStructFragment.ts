import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments, removeFromImportMap } from '@codama/fragments/rust';
import { isChildAttribute, type AttributeSpec, type NodeSpec, type Spec, type TypeExpr } from '@codama/spec';

import { getNodeMacroFlavor } from '../nodeFlavor';
import { getAttributeBodyLineFragment } from './attributeBodyLine';
import { use } from './helpers';

/**
 * Render the `#[<macro>] pub struct XxxNode { … }` declaration plus
 * any auto-derived `#[derive(...)]` line. The macro is one of
 * `#[node]` (default) or `#[type_node]` (for non-nestable type-category
 * nodes). Attributes are partitioned into "Data." and "Children."
 * sections; empty sections are omitted.
 *
 * Derive heuristic:
 *
 *   - `Copy`    when every attribute is Copy-able, or the struct is
 *               empty (see {@link isCopyAttribute}).
 *   - `Default` when every field is unconditionally Default-able (see
 *               {@link isUnconditionallyDefaultable}). Opaque required
 *               fields disqualify the struct.
 */
export function getNodeStructFragment(node: NodeSpec, spec: Spec): Fragment {
    const structName = pascalCase(node.kind);
    const { data, children } = partitionAttributes(node);
    const flavor = getNodeMacroFlavor(node, spec);
    const macroName = flavor === 'type_node' ? 'codama_nodes_derive::type_node' : 'codama_nodes_derive::node';
    const macros = fragment`#[${use(macroName)}]`;
    const derives = buildDeriveFragment(node, spec);
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

const SCALAR_KINDS: ReadonlySet<TypeExpr['kind']> = new Set(['integer', 'float', 'boolean', 'enumeration']);

function buildDeriveFragment(node: NodeSpec, spec: Spec): Fragment | undefined {
    const isEmpty = node.attributes.length === 0;
    const isCopy = isEmpty || node.attributes.every(a => isCopyAttribute(a, spec, new Set()));
    const isDefault = isEmpty || node.attributes.every(isUnconditionallyDefaultable);
    const derives: string[] = [];
    if (isCopy) derives.push('Copy');
    if (isDefault) derives.push('Default');
    if (derives.length === 0) return undefined;
    return fragment`#[derive(${derives.join(', ')})]`;
}

/**
 * `true` when this attribute's Rust type implements `Copy`. Scalar
 * kinds (`integer`/`float`/`boolean`/`enumeration`) always do —
 * `float` renders to the `Copy` `crate::Number`. A bare `node(...)`
 * reference is `Copy` exactly when the referenced node is itself a
 * `Copy` struct, resolved recursively against the spec; an `optional`
 * `node` stays `Copy` because `Option<T>` is `Copy` when `T` is.
 *
 * Every other kind is owned or heap-backed and disqualifies `Copy`:
 * `string`/`address`/`literal`/`codamaVersion` render to `String`,
 * `array` to `Vec`, `docs` to `Docs`, `tuple` to a tuple of those,
 * `nestedUnion` to a generic that isn't `Copy`, and `union`/`anyNode`
 * are boxed (`Box` is never `Copy`).
 *
 * `visited` guards the recursion against the self- and mutual
 * references the node graph contains (e.g. `instructionNode`).
 */
function isCopyAttribute(attr: AttributeSpec, spec: Spec, visited: Set<string>): boolean {
    if (SCALAR_KINDS.has(attr.type.kind)) return true;
    if (attr.type.kind !== 'node') return false;
    return isCopyNode(attr.type.name, spec, visited);
}

function isCopyNode(nodeKind: string, spec: Spec, visited: Set<string>): boolean {
    if (visited.has(nodeKind)) return false;
    const next = new Set(visited).add(nodeKind);
    const referenced = spec.categories.flatMap(c => c.nodes).find(n => n.kind === nodeKind);
    if (!referenced) return false;
    return referenced.attributes.every(a => isCopyAttribute(a, spec, next));
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
 * `nestedUnion`/`literalUnion` reference opaque types and disqualify
 * the struct. `float` renders to `crate::Number`, which is `Copy` but
 * NOT `Default`, so it's deliberately absent here.
 */
function isUnconditionallyDefaultable(attr: AttributeSpec): boolean {
    if (attr.optional === true) return true;
    const k = attr.type.kind;
    return (
        k === 'array' ||
        k === 'docs' ||
        k === 'boolean' ||
        k === 'integer' ||
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
