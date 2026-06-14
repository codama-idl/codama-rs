// Conformance fixture generator: produces input/expected JSON by running the
// REAL upstream `@codama/visitors` against known cases, so the Rust port can be
// verified against the TypeScript implementation's actual output.
//
// How to (re)generate `visitors.json`:
//   1. Have a codama clone built at the SAME @codama/spec version as codama-rs
//      (currently 1.6.0). From the clone root:
//        pnpm install --frozen-lockfile
//        node_modules/.bin/turbo run build --filter=@codama/visitors
//   2. Copy this file into the clone's `packages/visitors/` and run it there
//      (so `@codama/*` resolve), redirecting stdout to this fixtures dir:
//        cp generate.mjs <clone>/packages/visitors/_gen.mjs
//        (cd <clone>/packages/visitors && node _gen.mjs) > visitors.json
//        rm <clone>/packages/visitors/_gen.mjs
//
// The Rust harness (tests/conformance.rs) parses both `input` and `expected`
// into a `RootNode` and compares structurally, so JSON formatting / serde
// skip-default omissions don't matter.

import {
    accountNode,
    definedTypeLinkNode,
    definedTypeNode,
    numberTypeNode,
    programNode,
    rootNode,
    structFieldTypeNode,
    structTypeNode,
} from '@codama/nodes';
import { visit } from '@codama/visitors-core';
import { updateAccountsVisitor, updateDefinedTypesVisitor } from './dist/index.node.mjs';

const VISITORS = {
    updateAccounts: updateAccountsVisitor,
    updateDefinedTypes: updateDefinedTypesVisitor,
};

const cases = [];
const add = (visitor, label, args, input) => {
    const expected = visit(input, VISITORS[visitor](args));
    cases.push({ visitor, label, args, input, expected });
};

// ---- updateAccounts ----
const account = () =>
    rootNode(programNode({ accounts: [accountNode({ name: 'myAccount' })], name: 'myProgram', publicKey: '1111' }));
add('updateAccounts', 'rename', { myAccount: { name: 'myNewAccount' } }, account());
add('updateAccounts', 'resize', { myAccount: { size: 42 } }, account());
add('updateAccounts', 'delete', { myAccount: { delete: true } }, account());

const accountWithData = () =>
    rootNode(
        programNode({
            accounts: [
                accountNode({
                    data: structTypeNode([structFieldTypeNode({ name: 'myData', type: numberTypeNode('u32') })]),
                    name: 'myAccount',
                }),
            ],
            name: 'myProgram',
            publicKey: '1111',
        }),
    );
add('updateAccounts', 'renameDataField', { myAccount: { data: { myData: 'renamed' } } }, accountWithData());

// ---- updateDefinedTypes ----
const definedTypes = () =>
    rootNode(
        programNode({
            definedTypes: [
                definedTypeNode({ name: 'old', type: numberTypeNode('u32') }),
                definedTypeNode({
                    name: 'user',
                    type: structTypeNode([structFieldTypeNode({ name: 'ref', type: definedTypeLinkNode('old') })]),
                }),
            ],
            name: 'myProgram',
            publicKey: '1111',
        }),
    );
add('updateDefinedTypes', 'renameAndRewriteLinks', { old: { name: 'renamed' } }, definedTypes());
add('updateDefinedTypes', 'delete', { old: { delete: true } }, definedTypes());

const definedTypeWithFields = () =>
    rootNode(
        programNode({
            definedTypes: [
                definedTypeNode({
                    name: 'user',
                    type: structTypeNode([structFieldTypeNode({ name: 'a', type: numberTypeNode('u8') })]),
                }),
            ],
            name: 'myProgram',
            publicKey: '1111',
        }),
    );
add('updateDefinedTypes', 'renameInnerField', { user: { data: { a: 'b' } } }, definedTypeWithFields());

process.stdout.write(JSON.stringify(cases, null, 2) + '\n');
