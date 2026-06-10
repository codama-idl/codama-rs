import { type Fragment, mergeFragments } from '@codama/fragments/rust';
import type { NodeSpec } from '@codama/spec';

import type { CategoryRouting } from '../defaults';
import { getFromImplFragment } from './fromImpl';
import { getStructHasNameImplFragment } from './hasNameImpl';
import { getNodeStructFragment } from './nodeStructFragment';

/**
 * The body for one node's generated source file:
 *
 *   1. `#[node] pub struct XxxNode { … }` ({@link getNodeStructFragment}).
 *   2. `impl From<XxxNode> for crate::Node { … }` ({@link getFromImplFragment}).
 *   3. `impl HasName for XxxNode { … }` ({@link getStructHasNameImplFragment}),
 *      when applicable.
 */
export function getNodePageFragment(node: NodeSpec, routing: CategoryRouting): Fragment {
    const blocks: (Fragment | undefined)[] = [
        getNodeStructFragment(node),
        getFromImplFragment(node, routing),
        getStructHasNameImplFragment(node),
    ];
    return mergeFragments(blocks, parts => parts.join('\n\n'));
}
