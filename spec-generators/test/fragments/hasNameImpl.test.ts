import { attribute, defineNode, node, stringIdentifier } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import {
    getStructHasNameImplFragment,
    getUnionHasNameImplFragment,
    nodeHasName,
} from '../../src/fragments/hasNameImpl';

describe('nodeHasName', () => {
    it('is true when the node has a `name: stringIdentifier()` attribute', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        expect(nodeHasName(spec)).toBe(true);
    });

    it('is false when the node has no attributes at all', () => {
        const spec = defineNode('emptyNode', { attributes: [] });
        expect(nodeHasName(spec)).toBe(false);
    });

    it('is false when the node has a `name` attribute but it is not a stringIdentifier', () => {
        const spec = defineNode('exampleNode', { attributes: [attribute('name', node('programLinkNode'))] });
        expect(nodeHasName(spec)).toBe(false);
    });
});

describe('getStructHasNameImplFragment', () => {
    it('returns the HasName impl for a node with a name: stringIdentifier() attribute', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const result = getStructHasNameImplFragment(spec);
        // No inner indentation — rustfmt restores it.
        expect(result?.content).toBe(
            ['impl HasName for ProgramLinkNode {', 'fn name(&self) -> &CamelCaseString {', '&self.name', '}', '}'].join(
                '\n',
            ),
        );
    });

    it('carries the crate imports for HasName and CamelCaseString — and NOT the struct name (defined in the same file)', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const result = getStructHasNameImplFragment(spec);
        const imports = [...(result?.imports.keys() ?? [])].toSorted();
        expect(imports).toEqual(['crate::CamelCaseString', 'crate::HasName']);
    });

    it('returns undefined for a node without a name: stringIdentifier() attribute', () => {
        const spec = defineNode('emptyNode', { attributes: [] });
        expect(getStructHasNameImplFragment(spec)).toBeUndefined();
    });
});

describe('getUnionHasNameImplFragment', () => {
    const namedNode = (kind: string) => defineNode(kind, { attributes: [attribute('name', stringIdentifier())] });

    it('returns the union HasName impl when every member has a name attribute', () => {
        const variants = [
            { name: 'Account', node: namedNode('accountLinkNode') },
            { name: 'Program', node: namedNode('programLinkNode') },
        ];
        const result = getUnionHasNameImplFragment('LinkNode', variants);
        expect(result?.content).toBe(
            [
                'impl HasName for LinkNode {',
                'fn name(&self) -> &CamelCaseString {',
                'match self {',
                'LinkNode::Account(node) => node.name(),',
                'LinkNode::Program(node) => node.name(),',
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
        expect(getUnionHasNameImplFragment('LinkNode', variants)).toBeUndefined();
    });

    it('carries the crate imports for HasName and CamelCaseString', () => {
        const variants = [{ name: 'Account', node: namedNode('accountLinkNode') }];
        const result = getUnionHasNameImplFragment('LinkNode', variants);
        const imports = [...(result?.imports.keys() ?? [])].toSorted();
        expect(imports).toEqual(['crate::CamelCaseString', 'crate::HasName']);
    });
});
