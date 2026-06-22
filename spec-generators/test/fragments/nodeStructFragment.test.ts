import { type Spec } from '@codama/spec';
import {
    attribute,
    boolean,
    defineNode,
    enumeration,
    node,
    optionalAttribute,
    stringIdentifier,
    u64,
} from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getNodeStructFragment } from '../../src/fragments/nodeStructFragment';

// Empty Spec for tests that don't exercise type-category classification.
const EMPTY_SPEC: Spec = { version: '1.0.0', categories: [] };

describe('getNodeStructFragment', () => {
    it('emits the #[node] macro attribute and uses PascalCase struct name', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const result = getNodeStructFragment(spec, EMPTY_SPEC);
        expect(result.content).toContain('#[node]');
        expect(result.content).toContain('pub struct ProgramLinkNode {');
    });

    it('carries the codama_nodes_derive::node import so the page renderer brings the procedural macro into scope', () => {
        const spec = defineNode('programLinkNode', { attributes: [] });
        const result = getNodeStructFragment(spec, EMPTY_SPEC);
        expect([...result.imports.keys()]).toContain('codama_nodes_derive::node');
    });

    it('partitions data attributes before child attributes with section comments', () => {
        const spec = defineNode('accountLinkNode', {
            attributes: [optionalAttribute('program', node('programLinkNode')), attribute('name', stringIdentifier())],
        });
        const out = getNodeStructFragment(spec, EMPTY_SPEC).content;
        const dataIdx = out.indexOf('// Data.');
        const childrenIdx = out.indexOf('// Children.');
        expect(dataIdx).toBeGreaterThan(-1);
        expect(childrenIdx).toBeGreaterThan(-1);
        expect(dataIdx).toBeLessThan(childrenIdx);
        // No leading indentation on field lines — rustfmt restores it.
        expect(out).toContain('// Data.\npub name: CamelCaseString,');
        expect(out).toContain(
            [
                '// Children.',
                '#[serde(skip_serializing_if = "crate::is_default")]',
                'pub program: Option<ProgramLinkNode>,',
            ].join('\n'),
        );
    });

    it('omits the Children. section when the node has no child attributes', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const out = getNodeStructFragment(spec, EMPTY_SPEC).content;
        expect(out).toContain('// Data.');
        expect(out).not.toContain('// Children.');
    });

    it('omits the Data. section when the node has no data attributes', () => {
        const spec = defineNode('exampleNode', {
            attributes: [attribute('program', node('programLinkNode'))],
        });
        const out = getNodeStructFragment(spec, EMPTY_SPEC).content;
        expect(out).not.toContain('// Data.');
        expect(out).toContain('// Children.');
    });

    it('emits an empty `{}` struct body with #[derive(Copy, Default)] for an attribute-less node', () => {
        const spec = defineNode('remainderCountNode', { attributes: [] });
        const out = getNodeStructFragment(spec, EMPTY_SPEC).content;
        expect(out).toContain('#[derive(Copy, Default)]');
        expect(out).toContain('pub struct RemainderCountNode {}');
        expect(out).not.toContain('// Data.');
        expect(out).not.toContain('// Children.');
    });

    it('derives Copy when every attribute is a scalar kind (integer / float / boolean / enumeration)', () => {
        const spec = defineNode('fixedCountNode', { attributes: [attribute('value', u64())] });
        expect(getNodeStructFragment(spec, EMPTY_SPEC).content).toMatch(/#\[derive\(Copy(?:, Default)?\)\]/);
        const mixed = defineNode('numberTypeNode', {
            attributes: [
                attribute('format', enumeration('numberFormat')),
                attribute('endian', enumeration('endianness')),
                attribute('signed', boolean()),
            ],
        });
        // Required enumeration fields disqualify Default.
        expect(getNodeStructFragment(mixed, EMPTY_SPEC).content).toContain('#[derive(Copy)]');
        expect(getNodeStructFragment(mixed, EMPTY_SPEC).content).not.toContain('Default');
    });

    it('does NOT derive Copy when any attribute is a non-scalar (string / union / docs / …) or an unresolved node', () => {
        const withString = defineNode('fieldDiscriminatorNode', {
            attributes: [attribute('name', stringIdentifier()), attribute('offset', u64())],
        });
        const a = getNodeStructFragment(withString, EMPTY_SPEC).content;
        expect(a).not.toContain('#[derive(Copy');
        expect(a).toContain('#[derive(Default)]');
        // A `node` whose referent can't be resolved (empty spec) stays
        // conservatively non-Copy.
        const withNode = defineNode('constantDiscriminatorNode', {
            attributes: [attribute('offset', u64()), attribute('constant', node('constantValueNode'))],
        });
        expect(getNodeStructFragment(withNode, EMPTY_SPEC).content).not.toContain('#[derive(');
    });

    it('derives Copy when an optional `node` child references a node that is itself Copy', () => {
        // `stringDisplayNode` is scalar-only (two `Option<u64>`), so it's
        // Copy; a `stringTypeNode` that holds it as an optional child stays
        // Copy because `Option<T>` is Copy when `T` is.
        const displayNode = defineNode('stringDisplayNode', {
            attributes: [optionalAttribute('sliceStart', u64()), optionalAttribute('sliceEnd', u64())],
        });
        const hostNode = defineNode('stringTypeNode', {
            attributes: [
                attribute('encoding', enumeration('bytesEncoding')),
                optionalAttribute('display', node('stringDisplayNode')),
            ],
        });
        const spec: Spec = {
            version: '1.0.0',
            categories: [{ name: 'type', nodes: [displayNode], unions: [], enumerations: [], nestedUnions: [] }],
        };
        expect(getNodeStructFragment(hostNode, spec).content).toContain('#[derive(Copy)]');
    });

    it('does NOT derive Copy when a `node` child references a non-Copy node', () => {
        // `programLinkNode` carries a `CamelCaseString` (→ `String`), which is
        // not Copy, so a host holding it as a child is not Copy either.
        const childNode = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const hostNode = defineNode('exampleNode', {
            attributes: [attribute('flag', boolean()), optionalAttribute('link', node('programLinkNode'))],
        });
        const spec: Spec = {
            version: '1.0.0',
            categories: [{ name: 'link', nodes: [childNode], unions: [], enumerations: [], nestedUnions: [] }],
        };
        expect(getNodeStructFragment(hostNode, spec).content).not.toContain('#[derive(Copy');
    });

    it('derives `Default` when every required field is unconditionally Default-able', () => {
        const spec = defineNode('exampleNode', {
            attributes: [
                attribute('name', stringIdentifier()),
                attribute('flag', boolean()),
                optionalAttribute('docs', { kind: 'docs' }),
            ],
        });
        expect(getNodeStructFragment(spec, EMPTY_SPEC).content).toContain('#[derive(Default)]');
    });

    it('does NOT derive `Default` when a required field references an opaque type', () => {
        const enumSpec = defineNode('instructionStatusNode', {
            attributes: [attribute('lifecycle', enumeration('instructionLifecycle'))],
        });
        expect(getNodeStructFragment(enumSpec, EMPTY_SPEC).content).not.toContain('Default');
        const nodeSpec = defineNode('rootNode', {
            attributes: [attribute('program', node('programNode'))],
        });
        expect(getNodeStructFragment(nodeSpec, EMPTY_SPEC).content).not.toContain('Default');
    });
});
