import { pascalCase } from '@codama/fragments';
import { type AttributeSpec, isChildAttribute, type NodeSpec, type TypeExpr } from '@codama/spec';

import { getRustFieldName } from './naming';

/**
 * How a node attribute holds its child nodes, which decides how the generated code reads and rebuilds it.
 * It mirrors the field types of `attributeBodyLine.ts`:
 *
 *   - `node`      -> `X` or `Option<X>`, a node struct.
 *   - `union`     -> `Box<U>` or `Box<Option<U>>`, a node union.
 *   - `anyNode`   -> `Box<Node>`, any node.
 *   - `text`      -> `Text` or `Option<Text>`, a child only when it holds a `textNode`.
 *   - `docs`      -> `Docs`, a child only when it holds a `textNode`.
 *   - `array`     -> `Vec<X>` or `Vec<U>`, one child per item, whether optional or not.
 */
export type ChildSlot =
    | { readonly field: string; readonly kind: 'array'; readonly type: string }
    | { readonly field: string; readonly kind: 'docs' }
    | { readonly field: string; readonly kind: 'anyNode' }
    | { readonly field: string; readonly kind: 'node' | 'union'; readonly optional: boolean; readonly type: string }
    | { readonly field: string; readonly kind: 'text'; readonly optional: boolean };

/** A node attribute: either a child slot or plain data, in spec declaration order. */
export type NodeField = { readonly field: string; readonly kind: 'data' } | ChildSlot;

/** Every attribute of a node, base attributes included, as a child slot or plain data. */
export function getNodeFields(node: NodeSpec): readonly NodeField[] {
    return node.attributes.map(getNodeField);
}

/** The child slots of a node, in spec declaration order. */
export function getChildSlots(node: NodeSpec): readonly ChildSlot[] {
    return getNodeFields(node).filter((field): field is ChildSlot => field.kind !== 'data');
}

function getNodeField(attribute: AttributeSpec): NodeField {
    const field = getRustFieldName(attribute.name);
    const type = attribute.type;
    if (!isChildAttribute(type)) return { field, kind: 'data' };
    const optional = attribute.optional === true;
    switch (type.kind) {
        case 'docs':
            return { field, kind: 'docs' };
        case 'text':
            return { field, kind: 'text', optional };
        case 'anyNode':
            // Only a required `anyNode` exists in the spec, so the generated code handles no other shape.
            if (optional) throw new Error(`Unsupported optional anyNode attribute "${attribute.name}".`);
            return { field, kind: 'anyNode' };
        case 'node':
        case 'union':
            return { field, kind: type.kind, optional, type: pascalCase(type.name) };
        case 'array':
            return { field, kind: 'array', type: getArrayItemType(type.of) };
        default:
            throw new Error(`Unsupported child attribute type ${JSON.stringify(type)}.`);
    }
}

function getArrayItemType(type: TypeExpr): string {
    switch (type.kind) {
        case 'node':
        case 'union':
            return pascalCase(type.name);
        default:
            throw new Error(`Unsupported array item type ${JSON.stringify(type)} in a child attribute.`);
    }
}
