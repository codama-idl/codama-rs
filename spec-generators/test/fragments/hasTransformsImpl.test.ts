import { getSpec } from '@codama/spec';
import { array, attribute, node, optionalAttribute, union } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import {
    getStructHasTransformsImplFragment,
    isTransformsAttribute,
    nodeHasTransforms,
} from '../../src/fragments/hasTransformsImpl';

const spec = getSpec();
const nodeOf = (kind: string) => spec.categories.flatMap(c => c.nodes).find(n => n.kind === kind)!;

describe('isTransformsAttribute', () => {
    it('is true for an optional `transforms: array(union(transformNode))` attribute', () => {
        expect(isTransformsAttribute(optionalAttribute('transforms', array(union('transformNode'))))).toBe(true);
    });

    it('is true for a required `transforms: array(union(transformNode))` attribute', () => {
        expect(isTransformsAttribute(attribute('transforms', array(union('transformNode'))))).toBe(true);
    });

    it('is false for an array of transform nodes not named `transforms`', () => {
        expect(isTransformsAttribute(optionalAttribute('wrappers', array(union('transformNode'))))).toBe(false);
    });

    it('is false for a `transforms` attribute that is not an array', () => {
        expect(isTransformsAttribute(optionalAttribute('transforms', union('transformNode')))).toBe(false);
    });

    it('is false for a `transforms` array of another union', () => {
        expect(isTransformsAttribute(optionalAttribute('transforms', array(union('valueNode'))))).toBe(false);
    });

    it('is false for a `transforms` array of a single node kind', () => {
        expect(isTransformsAttribute(optionalAttribute('transforms', array(node('fixedSizeTransformNode'))))).toBe(
            false,
        );
    });
});

describe('nodeHasTransforms', () => {
    it('is true for a type node', () => {
        expect(nodeHasTransforms(nodeOf('integerTypeNode'))).toBe(true);
    });

    it('is true for the defined type link node', () => {
        expect(nodeHasTransforms(nodeOf('definedTypeLinkNode'))).toBe(true);
    });

    it('is false for a type category node without transforms', () => {
        expect(nodeHasTransforms(nodeOf('structFieldTypeNode'))).toBe(false);
    });

    it('is false for another link node', () => {
        expect(nodeHasTransforms(nodeOf('programLinkNode'))).toBe(false);
    });
});

describe('getStructHasTransformsImplFragment', () => {
    it('returns the HasTransforms impl for a node with a transforms attribute', () => {
        const result = getStructHasTransformsImplFragment(nodeOf('integerTypeNode'));
        expect(result?.content).toBe(
            [
                'impl HasTransforms for IntegerTypeNode {',
                'fn transforms(&self) -> &[TransformNode] {',
                '&self.transforms',
                '}',
                'fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {',
                '&mut self.transforms',
                '}',
                '}',
            ].join('\n'),
        );
    });

    it('carries the crate imports for HasTransforms and TransformNode and not the struct name', () => {
        const result = getStructHasTransformsImplFragment(nodeOf('definedTypeLinkNode'));
        const imports = [...(result?.imports.keys() ?? [])].toSorted();
        expect(imports).toEqual(['crate::HasTransforms', 'crate::TransformNode']);
    });

    it('returns undefined for a node without a transforms attribute', () => {
        expect(getStructHasTransformsImplFragment(nodeOf('programLinkNode'))).toBeUndefined();
    });
});
