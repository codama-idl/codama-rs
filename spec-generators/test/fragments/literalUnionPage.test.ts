import { describe, expect, it } from 'vitest';

import { getLiteralUnionPageFragment } from '../../src/fragments/literalUnionPage';

describe('getLiteralUnionPageFragment', () => {
    it('renders a `pub enum` with the non-serde derive set only', () => {
        const result = getLiteralUnionPageFragment({
            typeName: 'IsSigner',
            values: [true, false, 'either'],
        });
        // Non-serde derives only: bool-or-string serde is hand-written.
        expect(result.content).toContain('#[derive(Debug, PartialEq, Eq, Clone, Copy)]');
        expect(result.content).not.toContain('Serialize');
        expect(result.content).not.toContain('Deserialize');
        expect(result.content).not.toContain('Default');
        expect(result.content).not.toContain('rename_all');
        expect(result.content).toContain('pub enum IsSigner {');
    });

    it('emits variants in spec value order, mapping true/false to True/False and PascalCasing strings', () => {
        const result = getLiteralUnionPageFragment({
            typeName: 'IsSigner',
            values: [true, false, 'either'],
        });
        const variants = result.content.match(/^\s*(\w+),$/gm)?.map(s => s.trim().replace(',', '')) ?? [];
        expect(variants).toEqual(['True', 'False', 'Either']);
    });

    it('emits NO imports — the standard derives are in the Rust prelude', () => {
        const result = getLiteralUnionPageFragment({
            typeName: 'IsSigner',
            values: [true, false, 'either'],
        });
        expect([...result.imports.keys()]).toEqual([]);
    });
});
