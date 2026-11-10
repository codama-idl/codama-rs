import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { withBaseAttributes } from '../../src/baseAttributes';
import { getNodeEnumPageFragment, getNodeVariantName } from '../../src/fragments/nodeEnumPage';

// The generator appends the base attributes, such as `plugins`, to every node.
const spec = withBaseAttributes(getSpec());
const nodeOf = (kind: string) => spec.categories.flatMap(c => c.nodes).find(n => n.kind === kind)!;

describe('getNodeVariantName', () => {
    it('strips the trailing `Node` from the PascalCase kind', () => {
        expect(getNodeVariantName('accountNode')).toBe('Account');
        expect(getNodeVariantName('stringTypeNode')).toBe('StringType');
        expect(getNodeVariantName('fixedSizeTransformNode')).toBe('FixedSizeTransform');
    });
});

describe('getNodeEnumPageFragment', () => {
    const nodes = ['stringTypeNode', 'accountNode', 'pdaValueNode'].map(nodeOf);
    const result = getNodeEnumPageFragment(nodes);

    it('sorts the Node variants alphabetically, whatever the input order', () => {
        const body = result.content.slice(result.content.indexOf('pub enum Node {'));
        const variants = body.slice(0, body.indexOf('}')).match(/^(\w+)\(/gm);
        expect(variants).toEqual(['Account(', 'PdaValue(', 'StringType(']);
    });

    it('mirrors every Node variant in NodeRef over a borrowed node', () => {
        expect(result.content).toContain(
            "pub enum NodeRef<'a> {\nAccount(&'a AccountNode),\nPdaValue(&'a PdaValueNode),\nStringType(&'a StringTypeNode),\n}",
        );
    });

    it('converts each node struct from a Node and into a NodeRef', () => {
        for (const [type, variant, kind] of [
            ['AccountNode', 'Account', 'accountNode'],
            ['PdaValueNode', 'PdaValue', 'pdaValueNode'],
            ['StringTypeNode', 'StringType', 'stringTypeNode'],
        ]) {
            expect(result.content).toContain(`impl TryFrom<Node> for ${type} {`);
            expect(result.content).toContain(`Node::${variant}(node) => Ok(node),`);
            expect(result.content).toContain(`into: "${kind}".to_string(),`);
            expect(result.content).toContain(`impl<'a> From<&'a ${type}> for NodeRef<'a> {`);
        }
    });

    it('converts a NodeRef back into a reference to its node struct', () => {
        expect(result.content).toContain("impl<'a> TryFrom<NodeRef<'a>> for &'a StringTypeNode {");
        expect(result.content).toContain('NodeRef::StringType(node) => Ok(node),');
    });

    it('reads the identifier of the nodes that have one', () => {
        expect(result.content).toContain('Self::Account(node) => Some(&node.identifier),');
        expect(result.content).not.toContain('Self::StringType(node) => Some(&node.identifier),');
        expect(result.content).toContain('_ => None,');
    });

    it('reads the children of each node in the order of the spec attributes', () => {
        const arm = getArm(result.content, 'Self::PdaValue(node) => {');
        expect(arm).toContain('children.push(NodeRef::from(&*node.pda));');
        expect(arm.indexOf('node.pda')).toBeLessThan(arm.indexOf('node.seeds'));
        expect(arm).toContain('children.extend(node.seeds.iter().map(NodeRef::from));');
        expect(arm).toContain('children.extend(node.plugins.iter().map(NodeRef::from));');
    });

    it('rebuilds each node from its mapped children, deleting it when a required child is deleted', () => {
        const arm = getArm(result.content, 'Self::StringType(this) => {');
        expect(arm).toContain(
            'let display = map_optional::<StringDisplayNode>(this.display.as_ref().map(NodeRef::from), f)?;',
        );
        expect(arm).toContain('encoding: this.encoding.clone(),');
        expect(arm).toContain('display,');
        const pda = getArm(result.content, 'Self::PdaValue(this) => {');
        expect(pda).toContain(
            'let Some(pda) = map_required::<PdaValuePda>(NodeRef::from(&*this.pda), f)? else {\nreturn Ok(None);\n};',
        );
        expect(pda).toContain('pda: Box::new(pda),');
    });

    it('imports the helpers and slot types that the rebuild uses', () => {
        const imports = [...result.imports.keys()];
        expect(imports).toEqual(
            expect.arrayContaining([
                'codama_errors::CodamaError',
                'codama_errors::CodamaResult',
                'codama_nodes_derive::NodeUnion',
                'crate::HasKind',
                'crate::Identifier',
                'crate::node_ref::MapChild',
                'crate::node_ref::map_array',
                'crate::node_ref::map_optional',
                'crate::node_ref::map_required',
                'crate::PdaValuePda',
                'crate::StringDisplayNode',
                'derive_more::From',
            ]),
        );
    });
});

/** The text of the match arm that starts with `start`, up to the next arm at the same depth. */
function getArm(content: string, start: string): string {
    const from = content.indexOf(start);
    if (from === -1) throw new Error(`No arm starts with "${start}".`);
    let depth = 0;
    for (let index = from; index < content.length; index++) {
        if (content[index] === '{') depth++;
        if (content[index] === '}' && --depth === 0) return content.slice(from, index + 1);
    }
    throw new Error(`The arm "${start}" is not closed.`);
}
