import { fileURLToPath } from 'node:url';

import { joinPath, pathDirectory } from '@codama/fragments';

/**
 * Resolve the absolute path to the Rust monorepo root.
 *
 * The compiled bin lives at `<repoRoot>/spec-generators/dist/<entry>.mjs`;
 * resolving two levels up lands in the workspace root.
 */
export function getRepoDirectory(): string {
    const here = pathDirectory(fileURLToPath(import.meta.url));
    return joinPath(here, '..', '..');
}
