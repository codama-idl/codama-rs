import { attribute, boolean, docs, node, optionalAttribute, stringIdentifier } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getAttributeBodyLineFragment } from '../../src/fragments/attributeBodyLine';

describe('getAttributeBodyLineFragment', () => {
    it('renders a required primitive field with no serde attribute and no leading indentation', () => {
        const result = getAttributeBodyLineFragment(attribute('isWritable', boolean()));
        // No leading whitespace — rustfmt restores indentation on the
        // generated file.
        expect(result.content).toBe('pub is_writable: bool,');
    });

    it('renders a required stringIdentifier field as CamelCaseString with no serde attribute', () => {
        const result = getAttributeBodyLineFragment(attribute('name', stringIdentifier()));
        expect(result.content).toBe('pub name: CamelCaseString,');
    });

    it('renders an optional single-valued node attribute with Option<…> + skip_serializing_if', () => {
        const result = getAttributeBodyLineFragment(optionalAttribute('program', node('programLinkNode')));
        expect(result.content).toBe(
            ['#[serde(skip_serializing_if = "crate::is_default")]', 'pub program: Option<ProgramLinkNode>,'].join('\n'),
        );
    });

    it('renders a docs attribute with default + skip_serializing_if and no Option wrap', () => {
        const result = getAttributeBodyLineFragment(attribute('docs', docs()));
        expect(result.content).toBe(
            ['#[serde(default, skip_serializing_if = "crate::is_default")]', 'pub docs: Docs,'].join('\n'),
        );
    });

    it('escapes the Rust keyword `type` as `r#type` in field position', () => {
        const result = getAttributeBodyLineFragment(attribute('type', stringIdentifier()));
        expect(result.content).toBe('pub r#type: CamelCaseString,');
    });
});
