import { defineNode } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getFromImplFragment } from '../../src/fragments/fromImpl';

describe('getFromImplFragment', () => {
    it('emits the From<XxxNode> for crate::Node impl routing through the routing variant', () => {
        const spec = defineNode('accountLinkNode', { attributes: [] });
        const result = getFromImplFragment(spec, { mode: 'wrapped' as const, nodeVariant: 'Link' });
        // No inner indentation — rustfmt restores it.
        expect(result.content).toBe(
            [
                'impl From<AccountLinkNode> for crate::Node {',
                'fn from(val: AccountLinkNode) -> Self {',
                'crate::Node::Link(val.into())',
                '}',
                '}',
            ].join('\n'),
        );
    });

    it('carries no imports — crate::Node is referenced absolutely and the struct name is in scope', () => {
        const spec = defineNode('accountLinkNode', { attributes: [] });
        const result = getFromImplFragment(spec, { mode: 'wrapped' as const, nodeVariant: 'Link' });
        expect(result.imports.size).toBe(0);
    });

    it('uses the supplied routing variant verbatim (no normalisation)', () => {
        const spec = defineNode('someNode', { attributes: [] });
        const result = getFromImplFragment(spec, { mode: 'wrapped' as const, nodeVariant: 'Account' });
        expect(result.content).toContain('crate::Node::Account(val.into())');
    });
});
