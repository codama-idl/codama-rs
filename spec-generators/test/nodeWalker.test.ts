import { getSpec } from '@codama/spec';
import { anyNode, array, boolean, docs, json, node, stringIdentifier, text, union } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { withBaseAttributes } from '../src/baseAttributes';
import {
    getChildAttributes,
    getChildShape,
    getChildTarget,
    getNodeSpec,
    getVisitMethodName,
    getUnionEnums,
    getWalkerNodes,
    getWalkFunctionName,
    type UnionEnum,
} from '../src/nodeWalker';

const spec = getSpec();
const nodeOf = (kind: string) =>
    withBaseAttributes(spec)
        .categories.flatMap(c => c.nodes)
        .find(n => n.kind === kind)!;

describe('getChildShape', () => {
    it('reads a node, a union and anyNode as one child', () => {
        expect(getChildShape(node('programLinkNode'))).toBe('node');
        expect(getChildShape(union('typeNode'))).toBe('node');
        expect(getChildShape(anyNode())).toBe('node');
    });

    it('reads text and docs as a child that may be a plain string', () => {
        expect(getChildShape(text())).toBe('text');
        expect(getChildShape(docs())).toBe('text');
    });

    it('reads an array of nodes, unions or anyNode as one child per item', () => {
        expect(getChildShape(array(node('accountNode')))).toBe('array');
        expect(getChildShape(array(union('transformNode')))).toBe('array');
        expect(getChildShape(array(anyNode()))).toBe('array');
    });

    it('returns undefined for data attributes', () => {
        expect(getChildShape(boolean())).toBeUndefined();
        expect(getChildShape(stringIdentifier())).toBeUndefined();
        expect(getChildShape(json())).toBeUndefined();
        expect(getChildShape(array(boolean()))).toBeUndefined();
    });

    it('throws for a child attribute the walker cannot read', () => {
        expect(() => getChildShape(array(text()))).toThrow(/does not support the child attribute type/);
        expect(() => getChildShape(array(array(node('accountNode'))))).toThrow(
            /does not support the child attribute type/,
        );
    });
});

describe('getChildAttributes', () => {
    it('keeps the child attributes in declaration order, drops data attributes, and puts the base plugins last', () => {
        expect(getChildAttributes(nodeOf('structFieldTypeNode')).map(a => a.name)).toEqual([
            'docs',
            'type',
            'defaultValue',
            'display',
            'plugins',
        ]);
    });
});

describe('getChildTarget', () => {
    it('reads a node, or the items of an array of nodes, as a node of that kind', () => {
        expect(getChildTarget(node('programLinkNode'), spec)).toEqual({
            kind: 'node',
            node: getNodeSpec(spec, 'programLinkNode'),
        });
        expect(getChildTarget(array(node('accountNode')), spec)).toEqual({
            kind: 'node',
            node: getNodeSpec(spec, 'accountNode'),
        });
    });

    it('reads text and docs as a textNode', () => {
        const textNode = { kind: 'node', node: getNodeSpec(spec, 'textNode') };
        expect(getChildTarget(text(), spec)).toEqual(textNode);
        expect(getChildTarget(docs(), spec)).toEqual(textNode);
    });

    it('reads a union as its Rust enum, and anyNode as Node', () => {
        expect(getChildTarget(union('typeNode'), spec)).toEqual({ kind: 'union', name: 'TypeNode' });
        expect(getChildTarget(array(union('transformNode')), spec)).toEqual({ kind: 'union', name: 'TransformNode' });
        expect(getChildTarget(anyNode(), spec)).toEqual({ kind: 'union', name: 'Node' });
    });

    it('throws for a data attribute', () => {
        expect(() => getChildTarget(boolean(), spec)).toThrow(/holds no child node/);
    });
});

describe('getWalkerNodes', () => {
    it('returns every generated node, sorted by visit method name', () => {
        const kinds = getWalkerNodes(spec).map(n => n.kind);
        expect(kinds.slice(0, 3)).toEqual(['accountNode', 'accountBumpValueNode', 'accountDataValueNode']);
        expect(kinds.at(-1)).toBe('zeroableOptionTypeNode');
    });
});

describe('getUnionEnums', () => {
    const unions = getUnionEnums(spec);
    const generatedUnionOf = (name: string) =>
        unions.find((u): u is Extract<UnionEnum, { kind: 'generated' }> => u.kind === 'generated' && u.name === name)!;

    it('returns the external and the generated union enums, sorted by name', () => {
        expect(unions.slice(0, 3).map(u => u.name)).toEqual([
            'ConditionalValueCondition',
            'ConstantPdaSeedValue',
            'ContextualValueNode',
        ]);
        expect(unions.at(-1)?.name).toBe('ValueNode');
    });

    it('marks the hand-written enums as external', () => {
        expect(unions.filter(u => u.kind === 'external').map(u => u.name)).toEqual([
            'Node',
            'RegisteredTypeNode',
            'TypeNode',
        ]);
    });

    it('gives the variants of a generated enum with their node specs', () => {
        expect(generatedUnionOf('PdaValuePda').variants.map(v => [v.name, v.node.kind])).toEqual([
            ['Pda', 'pdaNode'],
            ['PdaLink', 'pdaLinkNode'],
        ]);
    });

    it('gives a RegisteredNodes-mode union its registered enum and its standalone twin', () => {
        const variantNames = (name: string) => generatedUnionOf(name).variants.map(v => v.name);
        expect(variantNames('RegisteredValueNode')).toEqual([...variantNames('ValueNode'), 'MapEntry', 'StructField']);
    });
});

describe('getVisitMethodName and getWalkFunctionName', () => {
    it('use the snake_case name without its Node suffix, and keep the name of Node', () => {
        expect(getVisitMethodName('arrayTypeNode')).toBe('visit_array_type');
        expect(getVisitMethodName('RegisteredValueNode')).toBe('visit_registered_value');
        expect(getVisitMethodName('ConstantPdaSeedValue')).toBe('visit_constant_pda_seed_value');
        expect(getVisitMethodName('Node')).toBe('visit_node');
        expect(getWalkFunctionName('arrayTypeNode')).toBe('walk_array_type');
        expect(getWalkFunctionName('Node')).toBe('walk_node');
    });
});
