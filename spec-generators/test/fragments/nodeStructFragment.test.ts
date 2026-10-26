import { getSpec } from '@codama/spec';
import {
    attribute,
    boolean,
    defineNode,
    docs,
    node,
    optionalAttribute,
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

    it('partitions data attributes before child attributes with section comments', () => {
        const out = getNodeStructFragment(nodeOf('accountLinkNode'), spec).content;
        const dataIdx = out.indexOf('// Data.');
        const childrenIdx = out.indexOf('// Children.');
        expect(dataIdx).toBeGreaterThan(-1);
        expect(childrenIdx).toBeGreaterThan(-1);
        expect(dataIdx).toBeLessThan(childrenIdx);
        // No leading indentation on field lines — rustfmt restores it.
        expect(out).toContain('// Data.\npub identifier: CamelCaseString,');
        expect(out).toContain(
            [
                '// Children.',
                '#[serde(skip_serializing_if = "crate::is_default")]',
                'pub program: Option<ProgramLinkNode>,',
            ].join('\n'),
        );
    });

    it('omits the Children. section when the node has no child attributes', () => {
        const dataOnly = defineNode('exampleNode', { attributes: [attribute('identifier', stringIdentifier())] });
        const out = getNodeStructFragment(dataOnly, spec).content;
        expect(out).toContain('// Data.');
        expect(out).not.toContain('// Children.');
    });

    it('omits the Data. section when the node has no data attributes', () => {
        const out = getNodeStructFragment(nodeOf('someValueNode'), spec).content;
        expect(out).not.toContain('// Data.');
        expect(out).toContain('// Children.');
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
