import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { withBaseAttributes } from '../../src/baseAttributes';
import {
    getChildVisitFragment,
    getNodeVisitMethodFragment,
    getNodeVisitorPageFragment,
    getNodeWalkFragment,
    getUnionVisitMethodFragment,
    getUnionWalkFragment,
} from '../../src/fragments/nodeVisitorPage';
import { getUnionEnums, getVisitMethodName, getWalkerNodes, getWalkFunctionName } from '../../src/nodeWalker';

const spec = withBaseAttributes(getSpec());
const nodeOf = (kind: string) => spec.categories.flatMap(c => c.nodes).find(n => n.kind === kind)!;
const attributeOf = (kind: string, name: string) => nodeOf(kind).attributes.find(a => a.name === name)!;
const unionOf = (name: string) => getUnionEnums(spec).find(u => u.name === name)!;
const childVisit = (kind: string, name: string) => getChildVisitFragment(attributeOf(kind, name), spec).content;

describe('getNodeVisitMethodFragment', () => {
    it('calls the walk function of the node kind by default', () => {
        const result = getNodeVisitMethodFragment(nodeOf('structTypeNode'));
        expect(result.content).toBe(
            [
                "fn visit_struct_type<'a>(&mut self, node: &'a StructTypeNode, path: &NodePath<'a>) {",
                'walk_struct_type(self, node, path);',
                '}',
            ].join('\n'),
        );
        expect([...result.imports.keys()].toSorted()).toEqual(['crate::NodePath', 'crate::StructTypeNode']);
    });
});

describe('getUnionVisitMethodFragment', () => {
    it('takes the segment and calls the generated walk function of the union', () => {
        const result = getUnionVisitMethodFragment(unionOf('CountNode'));
        expect(result.content).toBe(
            [
                "fn visit_count<'a>(&mut self, node: &'a CountNode, path: &NodePath<'a>, segment: PathSegment) {",
                'walk_count(self, node, path, segment);',
                '}',
            ].join('\n'),
        );
        expect([...result.imports.keys()].toSorted()).toEqual([
            'crate::CountNode',
            'crate::NodePath',
            'crate::PathSegment',
        ]);
    });

    it('imports the hand-written walk function of an external union', () => {
        const result = getUnionVisitMethodFragment(unionOf('TypeNode'));
        expect(result.content).toContain('walk_type(self, node, path, segment);');
        expect([...result.imports.keys()]).toContain('crate::walk_type');
    });
});

describe('getChildVisitFragment', () => {
    it('joins the path to a required node child', () => {
        expect(childVisit('booleanTypeNode', 'size')).toBe(
            'visitor.visit_integer_type(&node.size, &path.join(Attribute("size"), &node.size));',
        );
    });

    it('joins the identifier of a node kind that has one', () => {
        expect(childVisit('pdaLinkNode', 'program')).toBe(
            [
                'if let Some(child) = &node.program {',
                'visitor.visit_program_link(child, &path.join_named(Attribute("program"), child));',
                '}',
            ].join('\n'),
        );
    });

    it('gives a union child the parent path and the segment', () => {
        expect(childVisit('arrayTypeNode', 'item')).toBe('visitor.visit_type(&node.item, path, Attribute("item"));');
        expect(childVisit('providedNode', 'node')).toBe('visitor.visit_node(&node.node, path, Attribute("node"));');
    });

    it('reads an optional union through its Box', () => {
        expect(childVisit('structFieldTypeNode', 'defaultValue')).toBe(
            [
                'if let Some(child) = &*node.default_value {',
                'visitor.visit_value(child, path, Attribute("defaultValue"));',
                '}',
            ].join('\n'),
        );
    });

    it('visits text and docs only when the value is a TextNode', () => {
        const visit = 'visitor.visit_text(child, &path.join(Attribute("message"), child));';
        expect(childVisit('errorNode', 'message')).toBe(
            ['if let Text::Node(child) = &node.message {', visit, '}'].join('\n'),
        );
        expect(childVisit('instructionStatusNode', 'message')).toBe(
            ['if let Some(Text::Node(child)) = &node.message {', visit, '}'].join('\n'),
        );
        expect(childVisit('errorNode', 'docs')).toBe(
            [
                'if let Some(Text::Node(child)) = &*node.docs {',
                'visitor.visit_text(child, &path.join(Attribute("docs"), child));',
                '}',
            ].join('\n'),
        );
    });

    it('visits each item of a list with its index', () => {
        expect(childVisit('programNode', 'accounts')).toBe(
            [
                'for (index, child) in node.accounts.iter().enumerate() {',
                'visitor.visit_account(child, &path.join_named(Item("accounts", index), child));',
                '}',
            ].join('\n'),
        );
        expect(childVisit('arrayTypeNode', 'transforms')).toBe(
            [
                'for (index, child) in node.transforms.iter().enumerate() {',
                'visitor.visit_transform(child, path, Item("transforms", index));',
                '}',
            ].join('\n'),
        );
    });

    it('imports the segment variants it uses', () => {
        const result = getChildVisitFragment(attributeOf('errorNode', 'message'), spec);
        expect([...result.imports.keys()].toSorted()).toEqual(['crate::PathSegment::Attribute', 'crate::Text']);
    });
});

describe('getNodeWalkFragment', () => {
    it('enters the node, then visits the children in declaration order with the plugins last', () => {
        expect(getNodeWalkFragment(nodeOf('pdaLinkNode'), spec).content).toBe(
            [
                "pub fn walk_pda_link<'a, V: NodeVisitor + ?Sized>(visitor: &mut V, node: &'a PdaLinkNode, path: &NodePath<'a>) {",
                'visitor.enter_node("pdaLinkNode", path);',
                'if let Some(child) = &node.program {',
                'visitor.visit_program_link(child, &path.join_named(Attribute("program"), child));',
                '}',
                'for (index, child) in node.plugins.iter().enumerate() {',
                'visitor.visit_plugin(child, &path.join(Item("plugins", index), child));',
                '}',
                '}',
            ].join('\n'),
        );
    });
});

describe('getUnionWalkFragment', () => {
    it('visits the node inside the value with the path joined to it', () => {
        expect(getUnionWalkFragment(unionOf('PdaValuePda'))?.content).toBe(
            [
                "pub fn walk_pda_value_pda<'a, V: NodeVisitor + ?Sized>(visitor: &mut V, node: &'a PdaValuePda, path: &NodePath<'a>, segment: PathSegment) {",
                'match node {',
                'PdaValuePda::Pda(node) => visitor.visit_pda(node, &path.join_named(segment, node)),',
                'PdaValuePda::PdaLink(node) => visitor.visit_pda_link(node, &path.join_named(segment, node)),',
                '}',
                '}',
            ].join('\n'),
        );
    });

    it('leaves the walk function of an external union to its enum', () => {
        expect(getUnionWalkFragment(unionOf('TypeNode'))).toBeUndefined();
    });
});

describe('getNodeVisitorPageFragment', () => {
    const page = getNodeVisitorPageFragment(spec).content;
    const nodes = getWalkerNodes(spec);
    const unions = getUnionEnums(spec);
    const namesIn = (pattern: RegExp) => [...page.matchAll(pattern)].map(([, name]) => name).toSorted();

    it('declares the trait with the enter_node hook and one provided method per node kind and per union', () => {
        expect(page).toContain('pub trait NodeVisitor {');
        expect(page).toContain("fn enter_node<'a>(&mut self, _kind: &'static str, _path: &NodePath<'a>) {}");
        expect(namesIn(/^fn (visit_\w+)<'a>/gm)).toEqual(
            [...nodes.map(n => getVisitMethodName(n.kind)), ...unions.map(u => getVisitMethodName(u.name))].toSorted(),
        );
    });

    it('emits one walk function per node kind and per generated union', () => {
        expect(namesIn(/^pub fn (walk_\w+)</gm)).toEqual(
            [
                ...nodes.map(n => getWalkFunctionName(n.kind)),
                ...unions.filter(u => u.kind === 'generated').map(u => getWalkFunctionName(u.name)),
            ].toSorted(),
        );
    });
});
