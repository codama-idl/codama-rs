import { getSpec } from '@codama/spec';
import {
    attribute,
    boolean,
    defineNode,
    docs,
    node,
    optionalAttribute,
    string,
    stringIdentifier,
    text,
} from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getNodeStructFragment } from '../../src/fragments/nodeStructFragment';

const spec = getSpec();
const nodeOf = (kind: string) => spec.categories.flatMap(c => c.nodes).find(n => n.kind === kind)!;

describe('getNodeStructFragment', () => {
    it('emits the #[node] macro attribute and uses PascalCase struct name', () => {
        const result = getNodeStructFragment(nodeOf('programLinkNode'), spec);
        expect(result.content).toContain('#[node]');
        expect(result.content).toContain('pub struct ProgramLinkNode {');
    });

    it('carries the codama_nodes_derive::node import so the page renderer brings the procedural macro into scope', () => {
        const result = getNodeStructFragment(nodeOf('programLinkNode'), spec);
        expect([...result.imports.keys()]).toContain('codama_nodes_derive::node');
    });

    it('emits fields in declaration order without regrouping data and children', () => {
        // `docs` is a child declared between the `identifier` and `programId` data attributes.
        const exampleNode = defineNode('exampleNode', {
            attributes: [
                attribute('identifier', stringIdentifier()),
                optionalAttribute('docs', docs()),
                optionalAttribute('programId', string()),
            ],
        });
        const out = getNodeStructFragment(exampleNode, spec).content;
        const positions = ['pub identifier:', 'pub docs:', 'pub program_id:'].map(field => out.indexOf(field));
        expect(positions).not.toContain(-1);
        expect(positions).toEqual([...positions].sort((x, y) => x - y));
        // No leading indentation on field lines — rustfmt restores it.
        expect(out).toContain('{\npub identifier: Identifier,');
        expect(out).not.toContain('// Data.');
        expect(out).not.toContain('// Children.');
    });

    it('emits an empty `{}` struct body with #[derive(Default)] for an attribute-less node', () => {
        const out = getNodeStructFragment(defineNode('exampleNode', { attributes: [] }), spec).content;
        expect(out).toContain('#[derive(Default)]');
        expect(out).toContain('pub struct ExampleNode {}');
        expect(out).not.toContain('// Data.');
        expect(out).not.toContain('// Children.');
    });

    it('derives `Default` for a node of defaultable fields and nothing for an unresolved required node', () => {
        expect(getNodeStructFragment(nodeOf('sizeDiscriminatorNode'), spec).content).toContain('#[derive(Default)]');
        const withNode = defineNode('exampleNode', { attributes: [attribute('constant', node('unknownNode'))] });
        expect(getNodeStructFragment(withNode, spec).content).not.toContain('#[derive(');
    });

    it('derives `Default` when every required field is unconditionally Default-able', () => {
        const exampleNode = defineNode('exampleNode', {
            attributes: [
                attribute('identifier', stringIdentifier()),
                attribute('flag', boolean()),
                optionalAttribute('docs', docs()),
                optionalAttribute('label', text()),
            ],
        });
        expect(getNodeStructFragment(exampleNode, spec).content).toContain('#[derive(Default)]');
    });

    it('does NOT derive `Default` when a required field references an opaque type', () => {
        for (const kind of [
            'instructionStatusNode',
            'rootNode',
            'errorNode',
            'integerValueNode',
            'floatValueNode',
            'fieldDiscriminatorNode',
            'dataValueNode',
            'pluginNode',
        ]) {
            expect(getNodeStructFragment(nodeOf(kind), spec).content).not.toContain('Default');
        }
    });
});
