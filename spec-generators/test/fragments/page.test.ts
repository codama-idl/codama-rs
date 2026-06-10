import { addFragmentImports, fragment } from '@codama/fragments/rust';
import { describe, expect, it } from 'vitest';

import { getPageFragment } from '../../src/fragments/page';

describe('getPageFragment', () => {
    it('returns the input fragment unchanged when its imports map is empty', () => {
        const body = fragment`pub struct Foo;`;
        const result = getPageFragment(body);
        expect(result.imports.size).toBe(0);
        expect(result.content).toBe('pub struct Foo;');
    });

    it('emits a single `use crate::Foo;` line when only one crate import is present', () => {
        const body = addFragmentImports(fragment`pub type X = ProgramLinkNode;`, ['crate::ProgramLinkNode']);
        const result = getPageFragment(body);
        expect(result.content).toMatch(/^use crate::ProgramLinkNode;\n\npub type X/);
    });

    it('groups multiple crate:: imports into a single `use crate::{…}` line, sorted alphabetically', () => {
        const body = addFragmentImports(fragment`pub type X = Foo;`, [
            'crate::CamelCaseString',
            'crate::HasName',
            'crate::ProgramLinkNode',
        ]);
        const result = getPageFragment(body);
        expect(result.content).toMatch(/^use crate::\{CamelCaseString, HasName, ProgramLinkNode\};/);
    });

    it('keeps non-crate paths on their own use lines, sorted after the crate group', () => {
        const body = addFragmentImports(fragment`#[node]\npub struct Foo;`, ['codama_nodes_derive::node']);
        const result = getPageFragment(body);
        expect(result.content).toBe(['use codama_nodes_derive::node;', '', '#[node]', 'pub struct Foo;\n'].join('\n'));
    });

    it('ensures a single trailing newline on the rendered page', () => {
        const body = addFragmentImports(fragment`pub type X = Foo;`, ['crate::ProgramLinkNode']);
        const result = getPageFragment(body);
        expect(result.content.endsWith('\n')).toBe(true);
        expect(result.content.endsWith('\n\n')).toBe(false);
    });
});
