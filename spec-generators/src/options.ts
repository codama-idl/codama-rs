import { type Path } from '@codama/fragments';
import type { Spec } from '@codama/spec';

import { CATEGORY_DIRECTORIES } from './defaults';

/** User-facing options for the spec generator. */
export interface RenderOptions {
    /**
     * Map from each spec `category.name` to the output subdirectory
     * its entities are emitted into (relative to `generated/`). Use an
     * empty string for the top-level (no subdirectory). Omitted means
     * "use the v1 defaults" ({@link CATEGORY_DIRECTORIES}).
     */
    readonly categoryDirectories?: ReadonlyMap<string, string>;
    /** The spec major version this invocation targets. */
    readonly targetSpecMajor: number;
}

/** Options consumed by {@link generate}, the disk-writing entry point. */
export interface GenerateOptions extends RenderOptions {
    readonly outputDir: Path;
}

/** {@link RenderOptions} with every defaultable field resolved. */
export type ResolvedRenderOptions = Required<RenderOptions>;

/** Runtime context threaded through every fragment renderer. */
export type RenderScope = ResolvedRenderOptions;

export function resolveRenderOptions(options: RenderOptions): ResolvedRenderOptions {
    return {
        categoryDirectories: options.categoryDirectories ?? CATEGORY_DIRECTORIES,
        targetSpecMajor: options.targetSpecMajor,
    };
}

export function buildRenderScope(options: RenderOptions): RenderScope {
    return Object.freeze(resolveRenderOptions(options));
}

/**
 * Cross-check the caller-supplied options against the spec at
 * generation time. Catches a mismatched major version.
 */
export function validateRenderOptions(spec: Spec, options: RenderOptions): void {
    const actualMajor = parseSpecMajor(spec.version);
    if (actualMajor !== options.targetSpecMajor) {
        throw new Error(
            `targetSpecMajor=${options.targetSpecMajor} but the supplied spec is at version "${spec.version}" (major ${actualMajor}).`,
        );
    }
}

function parseSpecMajor(version: string): number {
    const match = /^(\d+)\./.exec(version);
    if (!match) throw new Error(`unable to parse spec version "${version}".`);
    return Number(match[1]);
}
