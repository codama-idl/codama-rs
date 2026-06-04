import { snakeCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { AttributeSpec, TypeExpr } from '@codama/spec';

import { getTypeExprFragment } from './typeExpr';

/** Spec attribute names that collide with Rust keywords and need `r#` escaping. */
const RUST_KEYWORDS: ReadonlySet<string> = new Set(['type']);

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
 */
export function getAttributeBodyLineFragment(attr: AttributeSpec): Fragment {
    const inner = getTypeExprFragment(attr.type);
    const isOptional = attr.optional === true;
    const isVecLike = isVecLikeType(attr.type);

    let typeFragment: Fragment;
    let serdeAttr: string;
    if (isOptional) {
        if (isVecLike) {
            typeFragment = inner;
            serdeAttr = '#[serde(default, skip_serializing_if = "crate::is_default")]';
        } else {
            typeFragment = fragment`Option<${inner}>`;
            serdeAttr = '#[serde(skip_serializing_if = "crate::is_default")]';
        }
    } else if (isDocsType(attr.type)) {
        // `docs` attributes are technically required by the spec but
        // their absence-in-JSON case needs explicit defaulting. Match
        // the hand-written convention.
        typeFragment = inner;
        serdeAttr = '#[serde(default, skip_serializing_if = "crate::is_default")]';
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

function isDocsType(type: TypeExpr): boolean {
    return type.kind === 'docs';
}

function rustFieldName(specName: string): string {
    const snake = snakeCase(specName);
    return RUST_KEYWORDS.has(snake) ? `r#${snake}` : snake;
}
