import { attribute, defineNode, node, optionalAttribute, stringIdentifier } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getNodeStructFragment } from '../../src/fragments/nodeStructFragment';

describe('getNodeStructFragment', () => {
    it('emits the #[node] macro attribute and uses PascalCase struct name', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const result = getNodeStructFragment(spec);
        expect(result.content).toContain('#[node]');
        expect(result.content).toContain('pub struct ProgramLinkNode {');
    });

    it('carries the codama_nodes_derive::node import so the page renderer brings the procedural macro into scope', () => {
        const spec = defineNode('programLinkNode', { attributes: [] });
        const result = getNodeStructFragment(spec);
        expect([...result.imports.keys()]).toContain('codama_nodes_derive::node');
    });

    it('partitions data attributes before child attributes with section comments', () => {
        const spec = defineNode('accountLinkNode', {
            attributes: [optionalAttribute('program', node('programLinkNode')), attribute('name', stringIdentifier())],
        });
        const out = getNodeStructFragment(spec).content;
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
        const out = getNodeStructFragment(spec).content;
        expect(out).toContain('// Data.');
        expect(out).not.toContain('// Children.');
    });

    it('omits the Data. section when the node has no data attributes', () => {
        const spec = defineNode('exampleNode', {
            attributes: [attribute('program', node('programLinkNode'))],
        });
        const out = getNodeStructFragment(spec).content;
        expect(out).not.toContain('// Data.');
        expect(out).toContain('// Children.');
    });
});
