import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getNodeMacroFlavor } from '../src/nodeFlavor';

describe('getNodeMacroFlavor', () => {
    const spec = getSpec();

    it('gives the type category 18 type_node and 2 node flavors', () => {
        const typeNodes = spec.categories.find(c => c.name === 'type')!.nodes;
        const nodeFlavored = typeNodes.filter(n => getNodeMacroFlavor(n, spec) === 'node').map(n => n.kind);
        expect(typeNodes).toHaveLength(20);
        expect(nodeFlavored.toSorted()).toEqual(['enumVariantTypeNode', 'structFieldTypeNode']);
    });

    it('gives node to every node outside the type category, including the typeNode leaf definedTypeLinkNode', () => {
        const others = spec.categories.filter(c => c.name !== 'type').flatMap(c => c.nodes);
        expect(others.map(n => n.kind)).toContain('definedTypeLinkNode');
        expect(others.filter(n => getNodeMacroFlavor(n, spec) !== 'node')).toEqual([]);
    });

    it('throws when the type category has no typeNode union', () => {
        const broken = {
            ...spec,
            categories: spec.categories.map(c =>
                c.name === 'type' ? { ...c, unions: c.unions.filter(u => u.name !== 'typeNode') } : c,
            ),
        };
        const booleanTypeNode = spec.categories.flatMap(c => c.nodes).find(n => n.kind === 'booleanTypeNode')!;
        expect(() => getNodeMacroFlavor(booleanTypeNode, broken)).toThrow(/no "typeNode" union/);
    });
});
