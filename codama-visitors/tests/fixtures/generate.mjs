// Conformance fixture generator: produces input/expected JSON by running the
// REAL upstream `@codama/visitors` against cases that mirror its own test suite,
// so the Rust port can be verified against the TypeScript implementation.
//
// How to (re)generate `visitors.json`:
//   1. Build a codama clone at the SAME @codama/spec version as codama-rs (1.6.0).
//      From the clone root:
//        pnpm install --frozen-lockfile
//        node_modules/.bin/turbo run build --filter=@codama/visitors
//   2. Run this file from the clone's `packages/visitors/` (so `@codama/*`
//      resolve), redirecting stdout here:
//        cp generate.mjs <clone>/packages/visitors/_gen.mjs
//        (cd <clone>/packages/visitors && node _gen.mjs) > visitors.json
//        rm <clone>/packages/visitors/_gen.mjs
//
// The Rust harness (tests/conformance.rs) parses both `input` and `expected`
// into a `RootNode` and compares structurally (so serde skip-default omissions
// don't matter), decoding each case's `args` into the matching Rust visitor.

import {
    accountNode,
    accountValueNode,
    argumentValueNode,
    arrayTypeNode,
    conditionalValueNode,
    constantPdaSeedNodeFromProgramId,
    constantPdaSeedNodeFromString,
    definedTypeLinkNode,
    definedTypeNode,
    enumTypeNode,
    eventNode,
    fixedCountNode,
    instructionAccountNode,
    instructionArgumentNode,
    instructionNode,
    noneValueNode,
    numberTypeNode,
    numberValueNode,
    optionTypeNode,
    pdaLinkNode,
    pdaNode,
    pdaSeedValueNode,
    pdaValueNode,
    programLinkNode,
    programNode,
    publicKeyTypeNode,
    rootNode,
    structFieldTypeNode,
    structTypeNode,
    variablePdaSeedNode,
} from '@codama/nodes';
import { LinkableDictionary, visit } from '@codama/visitors-core';
import {
    addPdasVisitor,
    fillDefaultPdaSeedValuesVisitor,
    getDefinedTypeHistogramVisitor,
    setStructDefaultValuesVisitor,
    unwrapDefinedTypesVisitor,
    unwrapInstructionArgsDefinedTypesVisitor,
    updateAccountsVisitor,
    updateDefinedTypesVisitor,
    updateInstructionsVisitor,
} from './dist/index.node.mjs';

// Maps a fixture's `visitor` name to a function that constructs the TS visitor
// from the (JSON-serializable) `args`.
const VISITORS = {
    addPdas: (a) => addPdasVisitor(a),
    setStructDefaultValues: (a) => setStructDefaultValuesVisitor(a),
    unwrapDefinedTypes: (a) => unwrapDefinedTypesVisitor(a),
    unwrapInstructionArgsDefinedTypes: () => unwrapInstructionArgsDefinedTypesVisitor(),
    updateAccounts: (a) => updateAccountsVisitor(a),
    updateDefinedTypes: (a) => updateDefinedTypesVisitor(a),
    updateInstructions: (a) => updateInstructionsVisitor(a),
};

const cases = [];
const add = (visitor, label, args, input) => {
    const expected = visit(input, VISITORS[visitor](args));
    cases.push({ visitor, label, args, input, expected });
};

// ============================ updateAccounts ============================
const ua_account = () =>
    rootNode(programNode({ accounts: [accountNode({ name: 'myAccount' })], name: 'myProgram', publicKey: '1111' }));
add('updateAccounts', 'rename', { myAccount: { name: 'myNewAccount' } }, ua_account());
add('updateAccounts', 'resize', { myAccount: { size: 42 } }, ua_account());
add('updateAccounts', 'delete', { myAccount: { delete: true } }, ua_account());

const ua_accountWithSamePda = () =>
    rootNode(programNode({
        accounts: [accountNode({ name: 'myAccount' })],
        name: 'myProgram', publicKey: '1111',
        pdas: [pdaNode({ name: 'myAccount', seeds: [] })],
    }));
add('updateAccounts', 'updatePdaWhenNameMatches',
    { myAccount: { seeds: [constantPdaSeedNodeFromString('utf8', 'myAccount')] } }, ua_accountWithSamePda());

const ua_accountLinkedToOtherPda = () =>
    rootNode(programNode({
        accounts: [accountNode({ name: 'myAccount', pda: pdaLinkNode('myPda') })],
        name: 'myProgram', publicKey: '1111',
        pdas: [pdaNode({ name: 'myPda', seeds: [] })],
    }));
add('updateAccounts', 'updateLinkedPdaWithSeeds',
    { myAccount: { seeds: [constantPdaSeedNodeFromString('utf8', 'myAccount')] } }, ua_accountLinkedToOtherPda());

const ua_accountNoPda = () =>
    rootNode(programNode({ accounts: [accountNode({ name: 'myAccount' })], name: 'myProgram', publicKey: '1111', pdas: [] }));
add('updateAccounts', 'createPdaWhenSeedsProvidedNoLink',
    { myAccount: { seeds: [constantPdaSeedNodeFromString('utf8', 'myAccount')] } }, ua_accountNoPda());

const ua_accountNoLinkWithExistingPda = () =>
    rootNode(programNode({
        accounts: [accountNode({ name: 'myAccount' })], name: 'myProgram', publicKey: '1111',
        pdas: [pdaNode({ name: 'myPda', seeds: [] })],
    }));
add('updateAccounts', 'updateExistingPdaWhenProvidingSeedsAndNewLink',
    { myAccount: { pda: pdaLinkNode('myPda'), seeds: [constantPdaSeedNodeFromString('utf8', 'myAccount')] } },
    ua_accountNoLinkWithExistingPda());
add('updateAccounts', 'createNewPdaWhenProvidingSeedsAndNewLink',
    { myAccount: { pda: pdaLinkNode('myPda'), seeds: [constantPdaSeedNodeFromString('utf8', 'myAccount')] } },
    ua_accountNoPda());
add('updateAccounts', 'renameAndCreatePda',
    { myAccount: { name: 'myNewAccount', seeds: [constantPdaSeedNodeFromString('utf8', 'myAccount')] } }, ua_account());

const ua_twoPrograms = () =>
    rootNode(
        programNode({ accounts: [accountNode({ name: 'candyMachine' })], name: 'myProgramA', publicKey: '1111' }),
        [programNode({ accounts: [accountNode({ name: 'candyMachine' })], name: 'myProgramB', publicKey: '2222' })],
    );
add('updateAccounts', 'renameMapsToProgram', { 'myProgramA.candyMachine': { name: 'newCandyMachine' } }, ua_twoPrograms());

const ua_accountWithData = () =>
    rootNode(programNode({
        accounts: [accountNode({
            name: 'myAccount',
            data: structTypeNode([structFieldTypeNode({ name: 'myData', type: numberTypeNode('u32') })]),
        })],
        name: 'myProgram', publicKey: '1111',
    }));
add('updateAccounts', 'renameDataFields', { myAccount: { data: { myData: 'myNewData' } } }, ua_accountWithData());

// ============================ updateDefinedTypes ============================
const udt_root = () =>
    rootNode(programNode({
        definedTypes: [
            definedTypeNode({ name: 'old', type: numberTypeNode('u32') }),
            definedTypeNode({ name: 'user', type: structTypeNode([structFieldTypeNode({ name: 'ref', type: definedTypeLinkNode('old') })]) }),
        ],
        name: 'myProgram', publicKey: '1111',
    }));
add('updateDefinedTypes', 'renameAndRewriteLinks', { old: { name: 'renamed' } }, udt_root());
add('updateDefinedTypes', 'delete', { old: { delete: true } }, udt_root());

const udt_innerRoot = () =>
    rootNode(programNode({
        definedTypes: [definedTypeNode({ name: 'user', type: structTypeNode([structFieldTypeNode({ name: 'a', type: numberTypeNode('u8') })]) })],
        name: 'myProgram', publicKey: '1111',
    }));
add('updateDefinedTypes', 'renameInnerField', { user: { data: { a: 'b' } } }, udt_innerRoot());

// ============================ updateInstructions ============================
add('updateInstructions', 'renameInstruction', { myInstruction: { name: 'myNewInstruction' } },
    rootNode(programNode({ instructions: [instructionNode({ name: 'myInstruction' })], name: 'myProgram', publicKey: '1111' })));

add('updateInstructions', 'renameInMultiProgram', { 'myProgramA.transfer': { name: 'newTransfer' } },
    rootNode(
        programNode({ instructions: [instructionNode({ name: 'transfer' })], name: 'myProgramA', publicKey: '1111' }),
        [programNode({ instructions: [instructionNode({ name: 'transfer' })], name: 'myProgramB', publicKey: '2222' })],
    ));

add('updateInstructions', 'renameAccount',
    { myInstruction: { accounts: { myAccount: { name: 'myNewAccount' } } } },
    rootNode(programNode({
        instructions: [instructionNode({
            accounts: [instructionAccountNode({ isSigner: false, isWritable: true, name: 'myAccount' })],
            name: 'myInstruction',
        })],
        name: 'myProgram', publicKey: '1111',
    })));

add('updateInstructions', 'renameArgument',
    { myInstruction: { arguments: { myArgument: { name: 'myNewArgument' } } } },
    rootNode(programNode({
        instructions: [instructionNode({
            arguments: [instructionArgumentNode({ name: 'myArgument', type: numberTypeNode('u8') })],
            name: 'myInstruction',
        })],
        name: 'myProgram', publicKey: '1111',
    })));

add('updateInstructions', 'setDefaultValue',
    { transferTokens: { arguments: { amount: { defaultValue: numberValueNode(1) } } } },
    rootNode(programNode({
        instructions: [instructionNode({
            arguments: [instructionArgumentNode({ name: 'amount', type: numberTypeNode('u64') })],
            name: 'transferTokens',
        })],
        name: 'myProgram', publicKey: '1111',
    })));

add('updateInstructions', 'setDefaultValueStrategy',
    { transferTokens: { arguments: {
        amount: { defaultValue: numberValueNode(1), defaultValueStrategy: 'optional' },
        discriminator: { defaultValue: numberValueNode(42), defaultValueStrategy: 'omitted' },
    } } },
    rootNode(programNode({
        instructions: [instructionNode({
            arguments: [
                instructionArgumentNode({ name: 'discriminator', type: numberTypeNode('u8') }),
                instructionArgumentNode({ name: 'amount', type: numberTypeNode('u64') }),
            ],
            name: 'transferTokens',
        })],
        name: 'myProgram', publicKey: '1111',
    })));

// ============================ setStructDefaultValues ============================
add('setStructDefaultValues', 'addsDefaultValuesToStructFields',
    { person: { age: numberValueNode(42), dateOfBirth: noneValueNode() } },
    rootNode(programNode({
        definedTypes: [definedTypeNode({
            name: 'person',
            type: structTypeNode([
                structFieldTypeNode({ name: 'age', type: numberTypeNode('u32') }),
                structFieldTypeNode({ name: 'dateOfBirth', type: optionTypeNode(numberTypeNode('i64')) }),
            ]),
        })],
        name: 'myProgram', publicKey: '1111',
    })));

add('setStructDefaultValues', 'addsDefaultValuesWithStrategies',
    { token: {
        delegateAuthority: { strategy: 'optional', value: noneValueNode() },
        discriminator: { strategy: 'omitted', value: numberValueNode(42) },
    } },
    rootNode(programNode({
        accounts: [accountNode({
            data: structTypeNode([
                structFieldTypeNode({ name: 'discriminator', type: numberTypeNode('u8') }),
                structFieldTypeNode({ name: 'delegateAuthority', type: optionTypeNode(publicKeyTypeNode()) }),
            ]),
            name: 'token',
        })],
        name: 'myProgram', publicKey: '1111',
    })));

add('setStructDefaultValues', 'addsDefaultValuesToInstructionArguments',
    { transferTokens: {
        amount: numberValueNode(1),
        discriminator: { strategy: 'omitted', value: numberValueNode(42) },
    } },
    rootNode(programNode({
        instructions: [instructionNode({
            arguments: [
                instructionArgumentNode({ name: 'discriminator', type: numberTypeNode('u8') }),
                instructionArgumentNode({ name: 'amount', type: numberTypeNode('u64') }),
            ],
            name: 'transferTokens',
        })],
        name: 'myProgram', publicKey: '1111',
    })));

// ============================ unwrapDefinedTypes ============================
add('unwrapDefinedTypes', 'unwrapsByFollowingLinks', ['myType'],
    rootNode(programNode({
        accounts: [accountNode({
            data: structTypeNode([structFieldTypeNode({ name: 'value', type: definedTypeLinkNode('myType') })]),
            name: 'myAccount',
        })],
        definedTypes: [definedTypeNode({ name: 'myType', type: numberTypeNode('u64') })],
        name: 'myProgram', publicKey: '1111',
    })));

add('unwrapDefinedTypes', 'followsLinkedNodesUsingCorrectPaths', ['typeB1', 'typeB2'],
    rootNode(
        programNode({
            definedTypes: [definedTypeNode({ name: 'typeA', type: definedTypeLinkNode('typeB1', programLinkNode('programB')) })],
            name: 'programA', publicKey: '1111',
        }),
        [programNode({
            definedTypes: [
                definedTypeNode({ name: 'typeB1', type: definedTypeLinkNode('typeB2') }),
                definedTypeNode({ name: 'typeB2', type: numberTypeNode('u64') }),
            ],
            name: 'programB', publicKey: '2222',
        })],
    ));

add('unwrapDefinedTypes', 'doesNotUnwrapTypesFromWrongPrograms', ['programA.myType'],
    rootNode(
        programNode({
            definedTypes: [
                definedTypeNode({ name: 'myType', type: numberTypeNode('u8') }),
                definedTypeNode({ name: 'myCopyType', type: definedTypeLinkNode('myType') }),
            ],
            name: 'programA', publicKey: '1111',
        }),
        [programNode({
            definedTypes: [
                definedTypeNode({ name: 'myType', type: numberTypeNode('u16') }),
                definedTypeNode({ name: 'myCopyType', type: definedTypeLinkNode('myType') }),
            ],
            name: 'programB', publicKey: '2222',
        })],
    ));

// ====================== unwrapInstructionArgsDefinedTypes ======================
add('unwrapInstructionArgsDefinedTypes', 'singleUse', {},
    rootNode(programNode({
        definedTypes: [
            definedTypeNode({ name: 'typeA', type: structTypeNode([structFieldTypeNode({ name: 'foo', type: numberTypeNode('u8') })]) }),
            definedTypeNode({ name: 'typeB', type: structTypeNode([structFieldTypeNode({ name: 'bar', type: numberTypeNode('u8') })]) }),
        ],
        instructions: [instructionNode({
            arguments: [instructionArgumentNode({ name: 'argA', type: definedTypeLinkNode('typeA') })],
            name: 'myInstruction',
        })],
        name: 'MyProgram', publicKey: '1111',
    })));

add('unwrapInstructionArgsDefinedTypes', 'multipleUses', {},
    rootNode(programNode({
        definedTypes: [
            definedTypeNode({ name: 'myType', type: structTypeNode([structFieldTypeNode({ name: 'foo', type: numberTypeNode('u8') })]) }),
            definedTypeNode({ name: 'myCopyType', type: definedTypeLinkNode('myType') }),
        ],
        instructions: [instructionNode({
            arguments: [instructionArgumentNode({ name: 'myArg', type: definedTypeLinkNode('myType') })],
            name: 'myInstruction',
        })],
        name: 'MyProgram', publicKey: '1111',
    })));

add('unwrapInstructionArgsDefinedTypes', 'nestedUsage', {},
    rootNode(programNode({
        definedTypes: [definedTypeNode({ name: 'myType', type: structTypeNode([structFieldTypeNode({ name: 'foo', type: numberTypeNode('u8') })]) })],
        instructions: [instructionNode({
            arguments: [instructionArgumentNode({ name: 'myArg', type: arrayTypeNode(definedTypeLinkNode('myType'), fixedCountNode(3)) })],
            name: 'myInstruction',
        })],
        name: 'MyProgram', publicKey: '1111',
    })));

add('unwrapInstructionArgsDefinedTypes', 'programBoundaries', {},
    rootNode(
        programNode({
            definedTypes: [definedTypeNode({ name: 'myType', type: numberTypeNode('u8') })],
            instructions: [instructionNode({
                arguments: [instructionArgumentNode({ name: 'myArg', type: definedTypeLinkNode('myType') })],
                name: 'myInstruction',
            })],
            name: 'programA', publicKey: '1111',
        }),
        [programNode({
            definedTypes: [
                definedTypeNode({ name: 'myType', type: numberTypeNode('u16') }),
                definedTypeNode({ name: 'myCopyType', type: definedTypeLinkNode('myType') }),
            ],
            name: 'programB', publicKey: '2222',
        })],
    ));

// ============================ addPdas ============================
const addPdas_existingPda = () =>
    pdaNode({
        name: 'associatedToken',
        seeds: [
            variablePdaSeedNode('owner', publicKeyTypeNode()),
            constantPdaSeedNodeFromProgramId(),
            variablePdaSeedNode('mint', publicKeyTypeNode()),
        ],
    });
const addPdas_program = () =>
    programNode({ name: 'myProgram', pdas: [addPdas_existingPda()], publicKey: 'Epo9rxh99jpeeWabRZi4tpgUVxZQeVn9vbbDjUztJtu4' });

add('addPdas', 'addsTwoPdas',
    { myProgram: [
        pdaNode({ name: 'metadata', seeds: [
            constantPdaSeedNodeFromString('utf8', 'metadata'),
            constantPdaSeedNodeFromProgramId(),
            variablePdaSeedNode('mint', publicKeyTypeNode()),
        ] }),
        pdaNode({ name: 'masterEdition', seeds: [
            constantPdaSeedNodeFromString('utf8', 'metadata'),
            constantPdaSeedNodeFromProgramId(),
            variablePdaSeedNode('mint', publicKeyTypeNode()),
            constantPdaSeedNodeFromString('utf8', 'edition'),
        ] }),
    ] },
    rootNode(addPdas_program()));

add('addPdas', 'addsPdasWithDocs',
    { myProgram: [
        pdaNode({ docs: 'Metadata for a token.', name: 'metadata', seeds: [
            constantPdaSeedNodeFromString('utf8', 'metadata'),
            constantPdaSeedNodeFromProgramId(),
            variablePdaSeedNode('mint', publicKeyTypeNode()),
        ] }),
        pdaNode({ docs: 'The master edition.', name: 'masterEdition', seeds: [
            constantPdaSeedNodeFromString('utf8', 'metadata'),
            constantPdaSeedNodeFromProgramId(),
            variablePdaSeedNode('mint', publicKeyTypeNode()),
            constantPdaSeedNodeFromString('utf8', 'edition'),
        ] }),
    ] },
    rootNode(addPdas_program()));

// ============================ getDefinedTypeHistogram ============================
// Non-node output (a usage map), so a separate section the Rust harness compares
// against the histogram returned by `get_defined_type_histogram`.
const histogram = [];
const addHistogram = (label, input) => {
    histogram.push({ label, input, expected: visit(input, getDefinedTypeHistogramVisitor()) });
};

addHistogram(
    'usageCounts',
    rootNode(programNode({
        accounts: [accountNode({
            data: structTypeNode([
                structFieldTypeNode({ name: 'field1', type: definedTypeLinkNode('myStruct') }),
                structFieldTypeNode({ name: 'field2', type: definedTypeLinkNode('myEnum') }),
            ]),
            name: 'myAccount',
        })],
        definedTypes: [
            definedTypeNode({ name: 'myStruct', type: structTypeNode([]) }),
            definedTypeNode({ name: 'myEnum', type: enumTypeNode([]) }),
        ],
        instructions: [instructionNode({
            arguments: [instructionArgumentNode({ name: 'arg1', type: definedTypeLinkNode('myStruct') })],
            name: 'myInstruction',
        })],
        name: 'customProgram', publicKey: '1111',
    })),
);

addHistogram(
    'eventPayloads',
    rootNode(programNode({
        definedTypes: [definedTypeNode({ name: 'eventPayload', type: structTypeNode([]) })],
        events: [eventNode({
            data: structTypeNode([structFieldTypeNode({ name: 'payload', type: definedTypeLinkNode('eventPayload') })]),
            name: 'payloadCreated',
        })],
        name: 'customProgram', publicKey: '1111',
    })),
);

addHistogram(
    'separatePrograms',
    rootNode(
        programNode({
            definedTypes: [
                definedTypeNode({ name: 'myType', type: numberTypeNode('u8') }),
                definedTypeNode({ name: 'myCopyType', type: definedTypeLinkNode('myType') }),
            ],
            name: 'programA', publicKey: '1111',
        }),
        [programNode({
            definedTypes: [
                definedTypeNode({ name: 'myType', type: numberTypeNode('u16') }),
                definedTypeNode({ name: 'myCopyType', type: definedTypeLinkNode('myType') }),
            ],
            name: 'programB', publicKey: '2222',
        })],
    ),
);

// ============================ fillDefaultPdaSeedValues ============================
// Operates on a value node given an instruction + linkables context, so a
// separate section carrying { program, instruction, value, strict, expected }.
const fillPda = [];
const addFillPda = (label, program, instruction, value, strict = false) => {
    const linkables = new LinkableDictionary();
    linkables.recordPath([program, program.pdas[0]]);
    const expected = visit(value, fillDefaultPdaSeedValuesVisitor([program, instruction], linkables, strict));
    fillPda.push({ label, program, instruction, value, strict, expected });
};

const fpda_program = () =>
    programNode({
        name: 'myProgram', publicKey: '1111',
        pdas: [pdaNode({
            name: 'myPda',
            seeds: [
                variablePdaSeedNode('seed1', numberTypeNode('u64')),
                variablePdaSeedNode('seed2', numberTypeNode('u64')),
                variablePdaSeedNode('seed3', publicKeyTypeNode()),
            ],
        })],
    });
const fpda_instruction = () =>
    instructionNode({
        accounts: [instructionAccountNode({ isSigner: false, isWritable: false, name: 'seed3' })],
        arguments: [instructionArgumentNode({ name: 'seed2', type: numberTypeNode('u64') })],
        name: 'myInstruction',
    });
const fpda_instructionArgOnly = () =>
    instructionNode({
        arguments: [instructionArgumentNode({ name: 'seed2', type: numberTypeNode('u64') })],
        name: 'myInstruction',
    });

addFillPda('fillsMissingSeeds', fpda_program(), fpda_instruction(),
    pdaValueNode('myPda', [pdaSeedValueNode('seed1', numberValueNode(42))]));

addFillPda('fillsNestedPdaValueNodes', fpda_program(), fpda_instruction(),
    conditionalValueNode({
        condition: accountValueNode('myAccount'),
        ifTrue: pdaValueNode('myPda', [pdaSeedValueNode('seed1', numberValueNode(42))]),
    }));

addFillPda('ignoresMissingSeeds', fpda_program(), fpda_instructionArgOnly(),
    pdaValueNode('myPda', [pdaSeedValueNode('seed1', numberValueNode(42))]));

process.stdout.write(JSON.stringify({ visitors: cases, histogram, fillPda }, null, 2) + '\n');
