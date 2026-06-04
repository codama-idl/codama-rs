import { snakeCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { AttributeSpec, TypeExpr } from '@codama/spec';

import { FIELD_TYPE_OVERRIDES } from '../defaults';
import { use } from './helpers';
import { getTypeExprFragment } from './typeExpr';

/** Spec attribute names that collide with Rust keywords and need `r#` escaping. */
const RUST_KEYWORDS: ReadonlySet<string> = new Set(['enum', 'type']);

/**
 * Render one spec attribute as a Rust struct field plus any preceding
 * `#[serde(…)]` attribute. The fragment carries the crate-rooted
 * imports referenced by the field type. Leading indentation is left
 * to `rustfmt`.
 *
 * Serde rules:
 *
 *   - Required, non-Vec field      → no `#[serde]` attr.
 *   - Optional single              → `Option<T>` + `#[serde(skip_serializing_if = "crate::is_default")]`.
 *   - Optional Vec                 → bare `Vec<T>` + `#[serde(default, skip_serializing_if = "crate::is_default")]`.
 *   - `docs` field                 → `Docs` + `#[serde(default, skip_serializing_if = "crate::is_default")]`.
 *
 * Box rule (box-all-union): every direct (non-`Vec`) field whose type
 * is a `union` is wrapped in `Box<…>`. Required → `Box<T>`; optional
 * → `Box<Option<T>>` (box outside Option). This keeps every
 * category-union variant pointer-sized for its union fields, which is
 * what `clippy::large_enum_variant` actually cares about. `array(…)`,
 * `nestedUnion`, `node`, and scalar fields are never boxed.
 */
export function getAttributeBodyLineFragment(nodeKind: string, attr: AttributeSpec): Fragment {
    const override = FIELD_TYPE_OVERRIDES.get(`${nodeKind}.${attr.name}`);
    const inner = override !== undefined ? use(override) : getTypeExprFragment(attr.type);
    const isOptional = attr.optional === true;
    const isVecLike = override !== undefined ? false : isVecLikeType(attr.type);
    const isUnion = override !== undefined ? false : isUnionType(attr.type);

    let typeFragment: Fragment;
    let serdeAttr: string;
    if (isDocsType(attr.type)) {
        // `docs` attributes are kept as a bare `Docs` regardless of
        // `optional`. `Docs` already wraps a `Vec<String>` with a
        // sensible `Default`/`is_default`, so wrapping it in
        // `Option<…>` would be redundant and divergent from the
        // hand-written convention.
        typeFragment = inner;
        serdeAttr = '#[serde(default, skip_serializing_if = "crate::is_default")]';
    } else if (isOptional) {
        if (isVecLike) {
            typeFragment = inner;
            serdeAttr = '#[serde(default, skip_serializing_if = "crate::is_default")]';
        } else if (isUnion) {
            // Optional union → `Box<Option<T>>` (box outside Option).
            typeFragment = fragment`Box<Option<${inner}>>`;
            serdeAttr = '#[serde(skip_serializing_if = "crate::is_default")]';
        } else {
            typeFragment = fragment`Option<${inner}>`;
            serdeAttr = '#[serde(skip_serializing_if = "crate::is_default")]';
        }
    } else if (isUnion) {
        typeFragment = fragment`Box<${inner}>`;
        serdeAttr = '';
    } else {
        typeFragment = inner;
        serdeAttr = '';
    }

    const fieldName = rustFieldName(attr.name);
    const fieldLine = fragment`pub ${fieldName}: ${typeFragment},`;
    if (serdeAttr === '') return fieldLine;
    return mergeFragments([fragment`${serdeAttr}`, fieldLine], parts => parts.join('\n'));
}

function isVecLikeType(type: TypeExpr): boolean {
    return type.kind === 'array';
}

function isUnionType(type: TypeExpr): boolean {
    return type.kind === 'union';
}

function isDocsType(type: TypeExpr): boolean {
    return type.kind === 'docs';
}

function rustFieldName(specName: string): string {
    const snake = snakeCase(specName);
    return RUST_KEYWORDS.has(snake) ? `r#${snake}` : snake;
}
