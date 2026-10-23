import {
    anyNode,
    array,
    attribute,
    boolean,
    defineNode,
    docs,
    node,
    optionalAttribute,
    stringIdentifier,
    union,
} from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getAttributeBodyLineFragment } from '../../src/fragments/attributeBodyLine';

describe('getAttributeBodyLineFragment', () => {
    const exampleNode = defineNode('exampleNode', { attributes: [] });

    it('renders a required primitive field with no serde attribute and no leading indentation', () => {
        const result = getAttributeBodyLineFragment(attribute('isWritable', boolean()), exampleNode);
        // No leading whitespace — rustfmt restores indentation on the
        // generated file.
        expect(result.content).toBe('pub is_writable: bool,');
    });

    it('renders a required stringIdentifier field as CamelCaseString with no serde attribute', () => {
        const result = getAttributeBodyLineFragment(attribute('variant', stringIdentifier()), exampleNode);
        expect(result.content).toBe('pub variant: CamelCaseString,');
    });

    // TODO(spec v2): expects the temporary serde rename. The next v2 regeneration drops it, update this expectation.
    it('renders the `name` identifier of a node as field `identifier` that serializes as `name`', () => {
        const nameAttribute = attribute('name', stringIdentifier());
        const owner = defineNode('pdaLinkNode', { attributes: [nameAttribute] });
        const result = getAttributeBodyLineFragment(nameAttribute, owner);
        expect(result.content).toBe(
            [
                '// TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.',
                '#[serde(rename = "name")]',
                'pub identifier: CamelCaseString,',
            ].join('\n'),
        );
    });

    // TODO(spec v2): checks the TEMP_NAME_NODE_EXCEPTIONS list. Update or delete when v2 drops it.
    it('keeps the field `name` for a node that keeps the Rust field `name`', () => {
        const nameAttribute = attribute('name', stringIdentifier());
        const owner = defineNode('pluginNode', { attributes: [nameAttribute] });
        const result = getAttributeBodyLineFragment(nameAttribute, owner);
        expect(result.content).toBe('pub name: CamelCaseString,');
    });

    it('renders an optional single-valued node attribute with Option<…> + skip_serializing_if', () => {
        const result = getAttributeBodyLineFragment(optionalAttribute('program', node('programLinkNode')), exampleNode);
        expect(result.content).toBe(
            ['#[serde(skip_serializing_if = "crate::is_default")]', 'pub program: Option<ProgramLinkNode>,'].join('\n'),
        );
    });

    it('renders a docs attribute as a bare `Docs` regardless of `optional`, never wrapping it in Option', () => {
        // `Docs` already has a sensible `Default` (empty Vec) + `is_default`,
        // so the spec's `optional: true` collapses to the same serde shape.
        const required = getAttributeBodyLineFragment(attribute('docs', docs()), exampleNode);
        const optional = getAttributeBodyLineFragment(optionalAttribute('docs', docs()), exampleNode);
        const expected = ['#[serde(default, skip_serializing_if = "crate::is_default")]', 'pub docs: Docs,'].join('\n');
        expect(required.content).toBe(expected);
        expect(optional.content).toBe(expected);
    });

    it('escapes the Rust keyword `type` as `r#type` in field position', () => {
        const result = getAttributeBodyLineFragment(attribute('type', stringIdentifier()), exampleNode);
        expect(result.content).toBe('pub r#type: CamelCaseString,');
    });

    it('boxes a required union field as `Box<T>` (box-all-union rule)', () => {
        const result = getAttributeBodyLineFragment(attribute('value', union('valueNode')), exampleNode);
        expect(result.content).toBe('pub value: Box<ValueNode>,');
    });

    it('boxes an optional union field as `Box<Option<T>>` (box outside Option)', () => {
        const result = getAttributeBodyLineFragment(optionalAttribute('value', union('valueNode')), exampleNode);
        expect(result.content).toBe(
            ['#[serde(skip_serializing_if = "crate::is_default")]', 'pub value: Box<Option<ValueNode>>,'].join('\n'),
        );
    });

    it('does NOT box a `Vec<union>` field — the Vec already heap-allocates', () => {
        const result = getAttributeBodyLineFragment(attribute('items', array(union('valueNode'))), exampleNode);
        expect(result.content).toBe(
            ['#[serde(default, skip_serializing_if = "crate::is_default")]', 'pub items: Vec<ValueNode>,'].join('\n'),
        );
    });

    it('gives a required array field the same skip-when-empty serde attrs as an optional one', () => {
        // Arrays are omitted when empty on write and default to `[]` when
        // absent on read, independent of `optional`. See the "Array
        // attributes are omitted when empty" convention in the `@codama/spec`
        // README.
        const required = getAttributeBodyLineFragment(attribute('accounts', array(node('accountNode'))), exampleNode);
        const optional = getAttributeBodyLineFragment(
            optionalAttribute('accounts', array(node('accountNode'))),
            exampleNode,
        );
        const expected = [
            '#[serde(default, skip_serializing_if = "crate::is_default")]',
            'pub accounts: Vec<AccountNode>,',
        ].join('\n');
        expect(required.content).toBe(expected);
        expect(optional.content).toBe(expected);
    });

    it('does NOT box a `node`-typed field (only `union`/`anyNode` triggers the box rule)', () => {
        const result = getAttributeBodyLineFragment(attribute('program', node('programLinkNode')), exampleNode);
        expect(result.content).toBe('pub program: ProgramLinkNode,');
    });

    it('boxes a required anyNode field as `Box<Node>` (anyNode follows the box-all-union rule)', () => {
        const result = getAttributeBodyLineFragment(attribute('node', anyNode()), exampleNode);
        expect(result.content).toBe('pub node: Box<Node>,');
    });

    it('boxes an optional anyNode field as `Box<Option<Node>>` (box outside Option)', () => {
        const result = getAttributeBodyLineFragment(optionalAttribute('node', anyNode()), exampleNode);
        expect(result.content).toBe(
            ['#[serde(skip_serializing_if = "crate::is_default")]', 'pub node: Box<Option<Node>>,'].join('\n'),
        );
    });
});
