import { getSpec } from '@codama/spec';
import { attribute, node, optionalAttribute, stringIdentifier } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import {
    getStructHasIdentifierImplFragment,
    getUnionHasIdentifierImplFragment,
    isIdentifierAttribute,
    nodeHasIdentifier,
} from '../../src/fragments/hasIdentifierImpl';

const spec = getSpec();
const nodeOf = (kind: string) => spec.categories.flatMap(c => c.nodes).find(n => n.kind === kind)!;

describe('isIdentifierAttribute', () => {
    it('is true for a required `identifier: stringIdentifier()` attribute', () => {
        expect(isIdentifierAttribute(attribute('identifier', stringIdentifier()))).toBe(true);
    });

    it('is false for an optional `identifier` attribute', () => {
        expect(isIdentifierAttribute(optionalAttribute('identifier', stringIdentifier()))).toBe(false);
    });

    it('is false for a stringIdentifier attribute not named `identifier`', () => {
        expect(isIdentifierAttribute(attribute('variant', stringIdentifier()))).toBe(false);
    });

    it('is false for an `identifier` attribute that is not a stringIdentifier', () => {
        expect(isIdentifierAttribute(attribute('identifier', node('programLinkNode')))).toBe(false);
    });
});

describe('nodeHasIdentifier', () => {
    it('is true when the node has a required `identifier: stringIdentifier()` attribute', () => {
        expect(nodeHasIdentifier(nodeOf('programLinkNode'))).toBe(true);
    });

    it('is false when the node has no attributes at all', () => {
        expect(nodeHasIdentifier(nodeOf('remainderCountNode'))).toBe(false);
    });

    it('is false when the identifier is optional', () => {
        expect(nodeHasIdentifier(nodeOf('publicKeyValueNode'))).toBe(false);
    });

    it('is false when the only stringIdentifier attribute has another name', () => {
        expect(nodeHasIdentifier(nodeOf('enumValueNode'))).toBe(false);
    });
});

describe('getStructHasIdentifierImplFragment', () => {
    it('returns the HasIdentifier impl for a node with a required identifier', () => {
        const result = getStructHasIdentifierImplFragment(nodeOf('programLinkNode'));
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
        const result = getStructHasIdentifierImplFragment(nodeOf('programLinkNode'));
        const imports = [...(result?.imports.keys() ?? [])].toSorted();
        expect(imports).toEqual(['crate::CamelCaseString', 'crate::HasIdentifier']);
    });

    it('returns undefined for a node without a required identifier', () => {
        expect(getStructHasIdentifierImplFragment(nodeOf('remainderCountNode'))).toBeUndefined();
    });
});

describe('getUnionHasIdentifierImplFragment', () => {
    it('returns the union HasIdentifier impl when every member has an identifier', () => {
        const variants = [
            { name: 'Account', node: nodeOf('accountLinkNode') },
            { name: 'Program', node: nodeOf('programLinkNode') },
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

    it('returns undefined when any member node lacks an identifier attribute', () => {
        const variants = [
            { name: 'Account', node: nodeOf('accountLinkNode') },
            { name: 'Remainder', node: nodeOf('remainderCountNode') },
        ];
        expect(getUnionHasIdentifierImplFragment('LinkNode', variants)).toBeUndefined();
    });

    it('returns undefined when any member node has an optional identifier', () => {
        const variants = [
            { name: 'Account', node: nodeOf('accountLinkNode') },
            { name: 'PublicKey', node: nodeOf('publicKeyValueNode') },
        ];
        expect(getUnionHasIdentifierImplFragment('LinkNode', variants)).toBeUndefined();
    });

    it('carries the crate imports for HasIdentifier and CamelCaseString', () => {
        const variants = [{ name: 'Account', node: nodeOf('accountLinkNode') }];
        const result = getUnionHasIdentifierImplFragment('LinkNode', variants);
        const imports = [...(result?.imports.keys() ?? [])].toSorted();
        expect(imports).toEqual(['crate::CamelCaseString', 'crate::HasIdentifier']);
    });
});
