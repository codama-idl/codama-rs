import { snakeCase } from '@codama/fragments';

/** Spec attribute names that collide with Rust keywords and need `r#` escaping. */
const RUST_KEYWORDS: ReadonlySet<string> = new Set(['enum', 'struct', 'type']);

/** The Rust field name of a spec attribute: `snakeCase`, with `r#` for a keyword. */
export function getRustFieldName(attributeName: string): string {
    const snake = snakeCase(attributeName);
    return RUST_KEYWORDS.has(snake) ? `r#${snake}` : snake;
}
