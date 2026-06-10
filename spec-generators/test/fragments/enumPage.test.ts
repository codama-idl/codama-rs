import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getEnumPageFragment } from '../../src/fragments/enumPage';

const spec = getSpec();
const sharedCategory = spec.categories.find(c => c.name === 'shared')!;
const programOrigin = sharedCategory.enumerations.find(e => e.name === 'programOrigin')!;
const instructionLifecycle = sharedCategory.enumerations.find(e => e.name === 'instructionLifecycle')!;
const endianness = sharedCategory.enumerations.find(e => e.name === 'endianness')!;
const defaultValueStrategy = sharedCategory.enumerations.find(e => e.name === 'defaultValueStrategy')!;

describe('getEnumPageFragment', () => {
    it('renders a plain `pub enum` with the standard derive set and `rename_all = "camelCase"`', () => {
        const result = getEnumPageFragment(programOrigin);
        expect(result.content).toContain('#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]');
        expect(result.content).toContain('#[serde(rename_all = "camelCase")]');
        expect(result.content).toContain('pub enum ProgramOrigin {');
    });

    it('emits variants in spec declaration order, PascalCased', () => {
        const result = getEnumPageFragment(instructionLifecycle);
        // Spec order is [archived, deprecated, draft, live] (alphabetical in the spec); preserved verbatim.
        const variants = result.content.match(/^\s*([A-Z]\w*),$/gm)?.map(s => s.trim().replace(',', '')) ?? [];
        expect(variants).toEqual(['Archived', 'Deprecated', 'Draft', 'Live']);
    });

    it('PascalCases lowercase spec names that need it (e.g. `be` → `Be`, not `BE`)', () => {
        // Endianness is the canonical "rename Rust to match spec" case.
        const result = getEnumPageFragment(endianness);
        expect(result.content).toContain('pub enum Endianness {');
        const variants = result.content.match(/^\s*([A-Z]\w*),$/gm)?.map(s => s.trim().replace(',', '')) ?? [];
        expect(variants).toEqual(['Be', 'Le']);
    });

    it('does NOT emit #[derive(Default)] — default-member choice is hand-written', () => {
        const result = getEnumPageFragment(defaultValueStrategy);
        // The derive list must not include Default; the docstring may legitimately contain the word.
        expect(result.content).toMatch(/#\[derive\(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize\)\]/);
        expect(result.content).not.toContain('Default,');
        expect(result.content).not.toContain('#[default]');
    });

    it('includes enum-level docs and variant-level docs from the spec', () => {
        const result = getEnumPageFragment(programOrigin);
        expect(result.content).toContain('/// The toolchain that originally generated a program description.');
        expect(result.content).toContain('/// The program was originally described by an Anchor IDL.');
        expect(result.content).toContain('/// The program was originally described by a Shank IDL.');
    });

    it('carries the serde import map entries for Serialize and Deserialize', () => {
        const result = getEnumPageFragment(programOrigin);
        const imports = [...result.imports.keys()].toSorted();
        expect(imports).toContain('serde::Serialize');
        expect(imports).toContain('serde::Deserialize');
    });
});
