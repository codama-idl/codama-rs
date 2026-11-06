import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { withBaseAttributes } from '../src/baseAttributes';

describe('withBaseAttributes', () => {
    const spec = getSpec();

    it('appends the base `plugins` attribute after the declared attributes of every node', () => {
        const declared = spec.categories.flatMap(c => c.nodes);
        const nodes = withBaseAttributes(spec).categories.flatMap(c => c.nodes);
        expect(nodes).toHaveLength(94);
        nodes.forEach((n, i) => {
            expect(n.attributes.map(a => a.name)).toEqual([...declared[i].attributes.map(a => a.name), 'plugins']);
        });
    });

    it('leaves the input spec untouched', () => {
        withBaseAttributes(spec);
        expect(spec.categories.flatMap(c => c.nodes).some(n => n.attributes.some(a => a.name === 'plugins'))).toBe(
            false,
        );
    });
});
