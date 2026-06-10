import { type Fragment, mergeFragments } from '@codama/fragments/rust';
import type { NodeSpec, Spec } from '@codama/spec';

import type { CategoryRouting } from '../defaults';
import { getFromImplFragment } from './fromImpl';
import { getStructHasNameImplFragment } from './hasNameImpl';
import { getNodeStructFragment } from './nodeStructFragment';

/**
 * The body for one node's generated source file: the struct, the
 * `From<…> for crate::Node` impl (wrapped routing only), and a
 * `HasName` impl when the node has a `name: stringIdentifier`.
 */
export function getNodePageFragment(node: NodeSpec, routing: CategoryRouting, spec: Spec): Fragment {
    const fromImpl = routing.mode === 'wrapped' ? getFromImplFragment(node, routing) : undefined;
    const blocks: (Fragment | undefined)[] = [
        getNodeStructFragment(node, spec),
        fromImpl,
        getStructHasNameImplFragment(node),
    ];
    return mergeFragments(blocks, parts => parts.join('\n\n'));
}
