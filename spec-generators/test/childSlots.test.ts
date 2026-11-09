import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { withBaseAttributes } from '../src/baseAttributes';
import { getChildSlots, getNodeFields, getRustFieldName } from '../src/childSlots';

const spec = withBaseAttributes(getSpec());
const nodeOf = (kind: string) => spec.categories.flatMap(c => c.nodes).find(n => n.kind === kind)!;

describe('getRustFieldName', () => {
    it('snake-cases the attribute and escapes Rust keywords', () => {
        expect(getRustFieldName('defaultValue')).toBe('default_value');
        expect(getRustFieldName('type')).toBe('r#type');
    });
});

describe('getChildSlots', () => {
    it('classifies every shape of child attribute, in spec order', () => {
        expect(getChildSlots(nodeOf('structFieldTypeNode'))).toEqual([
            { field: 'docs', kind: 'docs' },
            { field: 'r#type', kind: 'union', optional: false, type: 'TypeNode' },
            { field: 'default_value', kind: 'union', optional: true, type: 'ValueNode' },
            { field: 'display', kind: 'node', optional: true, type: 'StructFieldDisplayNode' },
            { field: 'plugins', kind: 'array', type: 'PluginNode' },
        ]);
        expect(getChildSlots(nodeOf('errorNode'))).toContainEqual({ field: 'message', kind: 'text', optional: false });
        expect(getChildSlots(nodeOf('providedNode'))).toContainEqual({ field: 'node', kind: 'anyNode' });
    });

    it('treats the plugins of every node as children', () => {
        for (const node of spec.categories.flatMap(c => c.nodes)) {
            expect(getChildSlots(node)).toContainEqual({ field: 'plugins', kind: 'array', type: 'PluginNode' });
        }
    });
});

describe('getNodeFields', () => {
    it('keeps data attributes as data, alongside the child slots', () => {
        const fields = getNodeFields(nodeOf('structFieldTypeNode'));
        expect(fields[0]).toEqual({ field: 'identifier', kind: 'data' });
        expect(fields.filter(field => field.kind === 'data').map(field => field.field)).toEqual([
            'identifier',
            'default_value_strategy',
        ]);
    });
});
