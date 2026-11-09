import { pascalCase, snakeCase } from '@codama/fragments';
import {
    type AttributeSpec,
    isChildAttribute,
    type NodeSpec,
    type Spec,
    type TypeExpr,
    type UnionSpec,
} from '@codama/spec';

import { CATEGORY_ROUTING } from './defaults';
import { getRegisteredUnionVariants } from './fragments/registeredUnionPage';
import { getUnionVariants, type UnionVariant } from './fragments/unionPage';
import { getEmittableUnions, isRegisteredCategoryUnion } from './unions';

/**
 * How the node walker reads the child nodes of an attribute that the spec marks as a child (`isChildAttribute`).
 *
 *   - `node`  -> `node`, `union` or `anyNode`: one child node.
 *   - `text`  -> `text` or `docs`: one child only when the value is a `textNode`, as a plain string holds no node.
 *   - `array` -> `array` of `node`, `union` or `anyNode`: one child per item.
 */
export type ChildShape = 'array' | 'node' | 'text';

/**
 * The {@link ChildShape} of an attribute type, or `undefined` for a data attribute.
 * Throws for a child attribute the walker cannot read, such as an array of text, so a spec bump cannot drop children.
 */
export function getChildShape(type: TypeExpr): ChildShape | undefined {
    if (!isChildAttribute(type)) return undefined;
    if (isNodeType(type)) return 'node';
    if (type.kind === 'text' || type.kind === 'docs') return 'text';
    if (type.kind === 'array' && isNodeType(type.of)) return 'array';
    throw new Error(`The node walker does not support the child attribute type ${JSON.stringify(type)}.`);
}

function isNodeType(type: TypeExpr): boolean {
    return type.kind === 'node' || type.kind === 'union' || type.kind === 'anyNode';
}

/** The child attributes of a node, in spec declaration order. Base attributes such as `plugins` come last. */
export function getChildAttributes(node: NodeSpec): readonly AttributeSpec[] {
    return node.attributes.filter(attr => getChildShape(attr.type) !== undefined);
}

/** What one child of an attribute is: a node of a known kind, or a union enum that only its value can resolve. */
export type ChildTarget =
    | { readonly kind: 'node'; readonly node: NodeSpec }
    | { readonly kind: 'union'; readonly name: string };

/** The {@link ChildTarget} of a child attribute type. A `text` or `docs` child is a `textNode`, `anyNode` is `Node`. */
export function getChildTarget(type: TypeExpr, spec: Spec): ChildTarget {
    const item = type.kind === 'array' ? type.of : type;
    switch (item.kind) {
        case 'node':
            return { kind: 'node', node: getNodeSpec(spec, item.name) };
        case 'text':
        case 'docs':
            return { kind: 'node', node: getNodeSpec(spec, 'textNode') };
        case 'union':
            return { kind: 'union', name: pascalCase(item.name) };
        case 'anyNode':
            return { kind: 'union', name: 'Node' };
        default:
            throw new Error(`The attribute type ${JSON.stringify(type)} holds no child node.`);
    }
}

/** The spec of a node kind. Throws for an unknown kind. */
export function getNodeSpec(spec: Spec, kind: string): NodeSpec {
    const node = spec.categories.flatMap(category => category.nodes).find(n => n.kind === kind);
    if (!node) throw new Error(`Unknown node kind "${kind}".`);
    return node;
}

/** Every node the generator emits a struct for, sorted by visit method name. */
export function getWalkerNodes(spec: Spec): readonly NodeSpec[] {
    return spec.categories
        .filter(category => CATEGORY_ROUTING.has(category.name))
        .flatMap(category => category.nodes)
        .toSorted((a, b) => getVisitMethodName(a.kind).localeCompare(getVisitMethodName(b.kind)));
}

/**
 * A union enum of the crate, such as `TypeNode`, which the walker dispatches to the node inside its value.
 *
 *   - `generated`: this generator emits the enum, so it knows the variants and emits its `walk_*` function too.
 *   - `external`: the enum is written by hand in `codama-nodes`, with its `walk_*` function next to it.
 */
export type UnionEnum =
    | { readonly kind: 'external'; readonly name: string }
    | { readonly kind: 'generated'; readonly name: string; readonly variants: readonly UnionVariant[] };

/** The external union enums: the `Node` registry, which `anyNode` maps to, and the `type` category unions. */
const EXTERNAL_UNION_ENUMS: readonly UnionEnum[] = [
    { kind: 'external', name: 'Node' },
    { kind: 'external', name: 'RegisteredTypeNode' },
    { kind: 'external', name: 'TypeNode' },
];

/** Every union enum of the crate, external or generated, sorted by name. */
export function getUnionEnums(spec: Spec): readonly UnionEnum[] {
    const generated = spec.categories
        .filter(category => {
            const routing = CATEGORY_ROUTING.get(category.name);
            return routing !== undefined && routing.emitUnions !== false;
        })
        .flatMap(category => getEmittableUnions(category, spec))
        .flatMap(union => getGeneratedUnionEnums(union, spec));
    return [...EXTERNAL_UNION_ENUMS, ...generated].toSorted((a, b) => a.name.localeCompare(b.name));
}

/** The enums the generator emits for one spec union: `<X>`, plus `Registered<X>` for a `RegisteredNodes`-mode union. */
function getGeneratedUnionEnums(union: UnionSpec, spec: Spec): readonly UnionEnum[] {
    const name = pascalCase(union.name);
    if (!isRegisteredCategoryUnion(union, spec)) {
        return [{ kind: 'generated', name, variants: getUnionVariants(union, spec) }];
    }
    const { standalone, registeredOnly } = getRegisteredUnionVariants(union, spec);
    const toVariant = (variant: { readonly kind: string; readonly name: string }): UnionVariant => ({
        name: variant.name,
        node: getNodeSpec(spec, variant.kind),
    });
    return [
        { kind: 'generated', name: `Registered${name}`, variants: [...standalone, ...registeredOnly].map(toVariant) },
        { kind: 'generated', name, variants: standalone.map(toVariant) },
    ];
}

/** The `NodeVisitor` method of a node kind or union enum: `arrayTypeNode` -> `visit_array_type`. */
export function getVisitMethodName(name: string): string {
    return `visit_${getWalkStem(name)}`;
}

/** The free function that walks a node kind or union enum: `arrayTypeNode` -> `walk_array_type`. */
export function getWalkFunctionName(name: string): string {
    return `walk_${getWalkStem(name)}`;
}

/** The snake_case name without its `Node` suffix. `Node` keeps it, so its name is not empty. */
function getWalkStem(name: string): string {
    const stem = name !== 'Node' && name.endsWith('Node') ? name.slice(0, -'Node'.length) : name;
    return snakeCase(stem);
}
