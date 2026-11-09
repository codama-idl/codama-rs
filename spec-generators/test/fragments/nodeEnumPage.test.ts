import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getNodeEnumPageFragment, getNodeVariantName } from '../../src/fragments/nodeEnumPage';

const spec = getSpec();
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

    it('imports the node structs, the derives and the error type it references', () => {
        expect([...result.imports.keys()].toSorted()).toEqual([
            'codama_errors::CodamaError',
            'codama_nodes_derive::NodeUnion',
            'crate::AccountNode',
            'crate::HasKind',
            'crate::PdaValueNode',
            'crate::StringTypeNode',
            'derive_more::From',
        ]);
    });
});
