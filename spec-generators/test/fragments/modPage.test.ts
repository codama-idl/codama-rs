import { createRenderMap, type Path } from '@codama/fragments';
import { type Fragment, fragment } from '@codama/fragments/rust';
import { describe, expect, it } from 'vitest';

import { getModPageFragment, getModPagesRenderMap, groupPathsByFolder } from '../../src/fragments/modPage';

function specPagesWithKeys(keys: readonly Path[]): ReadonlyMap<Path, Fragment> {
    const entries: Record<Path, Fragment> = {};
    for (const k of keys) entries[k] = fragment`// stub`;
    return createRenderMap(entries);
}

describe('getModPageFragment', () => {
    it('renders mod and pub use lines sorted alphabetically with a blank line between the two blocks', () => {
        const result = getModPageFragment(['b', 'a', 'c']);
        expect(result.content).toBe(
            ['mod a;', 'mod b;', 'mod c;', '', 'pub use a::*;', 'pub use b::*;', 'pub use c::*;\n'].join('\n'),
        );
    });
});

describe('groupPathsByFolder', () => {
    it('strips the .rs extension and groups by parent folder', () => {
        const grouped = groupPathsByFolder(['a.rs', 'sub/b.rs', 'sub/c.rs']);
        expect(grouped.get('')).toEqual(['a']);
        expect(grouped.get('sub')).toEqual(['b', 'c']);
    });

    it('places extension-less paths and top-level paths under the empty-string sentinel folder', () => {
        const grouped = groupPathsByFolder(['top']);
        expect(grouped.get('')).toEqual(['top']);
    });
});

describe('getModPagesRenderMap', () => {
    it('emits one mod.rs per subdirectory plus a root mod.rs that re-exports every subdirectory', () => {
        const specPages = specPagesWithKeys(['link_nodes/account_link_node.rs', 'link_nodes/program_link_node.rs']);
        const map = getModPagesRenderMap(specPages);
        expect(map.has('link_nodes/mod.rs')).toBe(true);
        expect(map.has('mod.rs')).toBe(true);
        const linkMod = map.get('link_nodes/mod.rs')!.content;
        expect(linkMod).toContain('mod account_link_node;');
        expect(linkMod).toContain('pub use program_link_node::*;');
        const rootMod = map.get('mod.rs')!.content;
        expect(rootMod).toContain('mod link_nodes;');
        expect(rootMod).toContain('pub use link_nodes::*;');
    });

    it('only emits a root mod.rs (no subdirectory mod.rs) when every spec page is top-level', () => {
        const specPages = specPagesWithKeys(['account_node.rs', 'program_node.rs']);
        const map = getModPagesRenderMap(specPages);
        expect([...map.keys()].toSorted()).toEqual(['mod.rs']);
        const rootMod = map.get('mod.rs')!.content;
        expect(rootMod).toContain('mod account_node;');
        expect(rootMod).toContain('pub use program_node::*;');
    });
});
