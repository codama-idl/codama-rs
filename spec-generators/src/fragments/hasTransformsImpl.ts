import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { AttributeSpec, NodeSpec } from '@codama/spec';

import { use } from './helpers';

/**
 * `true` for a `transforms: array(union('transformNode'))` attribute, rendered as `transforms: Vec<TransformNode>`.
 * Optional or not, an array renders as a bare `Vec`, so `HasTransforms::transforms()` can always return it.
 */
export function isTransformsAttribute(attr: AttributeSpec): boolean {
    return (
        attr.name === 'transforms' &&
        attr.type.kind === 'array' &&
        attr.type.of.kind === 'union' &&
        attr.type.of.name === 'transformNode'
    );
}

/** `true` when the node has a transforms attribute, the only condition for a `HasTransforms` impl. */
export function nodeHasTransforms(node: NodeSpec): boolean {
    return node.attributes.some(isTransformsAttribute);
}

/**
 * Render the `impl HasTransforms for XxxNode` block for a struct, or
 * `undefined` when the node doesn't satisfy {@link nodeHasTransforms}.
 *
 * The struct's own name is interpolated (not `use`d) because the
 * struct lives in the same file.
 */
export function getStructHasTransformsImplFragment(node: NodeSpec): Fragment | undefined {
    if (!nodeHasTransforms(node)) return undefined;
    const structName = pascalCase(node.kind);
    const transformNode = use('crate::TransformNode');
    return mergeFragments(
        [
            fragment`impl ${use('crate::HasTransforms')} for ${structName} {`,
            fragment`fn transforms(&self) -> &[${transformNode}] {`,
            fragment`&self.transforms`,
            fragment`}`,
            fragment`fn transforms_mut(&mut self) -> &mut Vec<${transformNode}> {`,
            fragment`&mut self.transforms`,
            fragment`}`,
            fragment`}`,
        ],
        lines => lines.join('\n'),
    );
}
