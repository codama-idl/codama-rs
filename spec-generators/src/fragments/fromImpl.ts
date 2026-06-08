import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment } from '@codama/fragments/rust';
import type { NodeSpec } from '@codama/spec';

import type { CategoryRouting } from '../defaults';

/**
 * Render `impl From<XxxNode> for crate::Node` routing through the
 * category union (`wrapped` mode only; `direct` nodes get their
 * `From` from the `Node` enum's `#[derive(From)]`).
 */
export function getFromImplFragment(node: NodeSpec, routing: Extract<CategoryRouting, { mode: 'wrapped' }>): Fragment {
    const structName = pascalCase(node.kind);
    return fragment`impl From<${structName}> for crate::Node {\nfn from(val: ${structName}) -> Self {\ncrate::Node::${routing.nodeVariant}(val.into())\n}\n}`;
}
