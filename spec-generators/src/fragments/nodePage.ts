import { type Fragment, mergeFragments } from '@codama/fragments/rust';
import type { NodeSpec, Spec } from '@codama/spec';

import { getStructHasIdentifierImplFragment } from './hasIdentifierImpl';
import { getStructHasTransformsImplFragment } from './hasTransformsImpl';
import { getNodeStructFragment } from './nodeStructFragment';

/**
 * The body for one node's generated source file: the struct, a
 * `HasIdentifier` impl when the node has a required identifier, and a
 * `HasTransforms` impl when the node has a transforms attribute. Its
 * conversions to and from `Node` live in the generated `node.rs`.
 */
export function getNodePageFragment(node: NodeSpec, spec: Spec): Fragment {
    const blocks: (Fragment | undefined)[] = [
        getNodeStructFragment(node, spec),
        getStructHasIdentifierImplFragment(node),
        getStructHasTransformsImplFragment(node),
    ];
    return mergeFragments(blocks, parts => parts.join('\n\n'));
}
