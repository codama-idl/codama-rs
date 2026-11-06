import { attribute, defineNode, node, stringIdentifier } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import {
    getStructHasIdentifierImplFragment,
    getUnionHasIdentifierImplFragment,
    nodeHasIdentifier,
} from '../../src/fragments/hasNameImpl';

describe('nodeHasIdentifier', () => {
    it('is true when the node has a `name: stringIdentifier()` attribute', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        expect(nodeHasIdentifier(spec)).toBe(true);
    });

    it('is false when the node has no attributes at all', () => {
        const spec = defineNode('emptyNode', { attributes: [] });
        expect(nodeHasIdentifier(spec)).toBe(false);
    });

    it('is false when the node has a `name` attribute but it is not a stringIdentifier', () => {
        const spec = defineNode('exampleNode', { attributes: [attribute('name', node('programLinkNode'))] });
        expect(nodeHasIdentifier(spec)).toBe(false);
    });
});

describe('getStructHasIdentifierImplFragment', () => {
    it('returns the HasIdentifier impl for a node with a name: stringIdentifier() attribute', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const result = getStructHasIdentifierImplFragment(spec);
        // No inner indentation — rustfmt restores it.
        expect(result?.content).toBe(
            [
                'impl HasIdentifier for ProgramLinkNode {',
                'fn identifier(&self) -> &CamelCaseString {',
                '&self.identifier',
                '}',
                '}',
            ].join('\n'),
        );
    });

    it('carries the crate imports for HasIdentifier and CamelCaseString - and NOT the struct name (defined in the same file)', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const result = getStructHasIdentifierImplFragment(spec);
        const imports = [...(result?.imports.keys() ?? [])].toSorted();
        expect(imports).toEqual(['crate::CamelCaseString', 'crate::HasIdentifier']);
    });

    it('returns undefined for a node without a name: stringIdentifier() attribute', () => {
        const spec = defineNode('emptyNode', { attributes: [] });
        expect(getStructHasIdentifierImplFragment(spec)).toBeUndefined();
    });
});

describe('getUnionHasIdentifierImplFragment', () => {
    const namedNode = (kind: string) => defineNode(kind, { attributes: [attribute('name', stringIdentifier())] });

    it('returns the union HasIdentifier impl when every member has an identifier', () => {
        const variants = [
            { name: 'Account', node: namedNode('accountLinkNode') },
            { name: 'Program', node: namedNode('programLinkNode') },
        ];
        const result = getUnionHasIdentifierImplFragment('LinkNode', variants);
        expect(result?.content).toBe(
            [
                'impl HasIdentifier for LinkNode {',
                'fn identifier(&self) -> &CamelCaseString {',
                'match self {',
                'LinkNode::Account(node) => node.identifier(),',
                'LinkNode::Program(node) => node.identifier(),',
                '}',
                '}',
                '}',
            ].join('\n'),
        );
    });

    it('returns undefined when any member node lacks a name attribute', () => {
        const variants = [
            { name: 'Account', node: namedNode('accountLinkNode') },
            { name: 'Empty', node: defineNode('emptyNode', { attributes: [] }) },
        ];
        expect(getUnionHasIdentifierImplFragment('LinkNode', variants)).toBeUndefined();
    });

    it('carries the crate imports for HasIdentifier and CamelCaseString', () => {
        const variants = [{ name: 'Account', node: namedNode('accountLinkNode') }];
        const result = getUnionHasIdentifierImplFragment('LinkNode', variants);
        const imports = [...(result?.imports.keys() ?? [])].toSorted();
        expect(imports).toEqual(['crate::CamelCaseString', 'crate::HasIdentifier']);
    });
});
