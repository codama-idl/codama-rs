import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment } from '@codama/fragments/rust';
import type { NodeSpec } from '@codama/spec';

import type { CategoryRouting } from '../defaults';

/**
 * Render the `impl From<XxxNode> for crate::Node` block that routes
 * the node through its category's union variant (see
 * {@link CategoryRouting}).
 *
 * `crate::Node` is written with its absolute path, so no import is
 * added to the fragment's import map.
 */
export function getFromImplFragment(node: NodeSpec, routing: CategoryRouting): Fragment {
    const structName = pascalCase(node.kind);
    return fragment`impl From<${structName}> for crate::Node {\nfn from(val: ${structName}) -> Self {\ncrate::Node::${routing.nodeVariant}(val.into())\n}\n}`;
}
