import { createRenderMap, type Path, pathBasename, pathDirectory, type RenderMap } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';

/**
 * Build the per-folder and root `mod.rs` re-export pages from a set of
 * already-emitted spec pages. Each `mod.rs` lists subdirectory and
 * file siblings via `mod xxx;` + `pub use xxx::*;` blocks.
 *
 * Top-level files (directly under `generated/`) plus subdirectories
 * (one per spec category that the generator emits) flow into the root
 * `mod.rs`. Each subdirectory gets its own `mod.rs` listing its
 * per-node and per-union files.
 */
export function getModPagesRenderMap(specPages: RenderMap<Fragment>): RenderMap<Fragment> {
    const filesByFolder = groupPathsByFolder([...specPages.keys()]);
    const entries: Record<Path, Fragment> = {};

    const topLevelFiles = filesByFolder.get('') ?? [];
    const subdirs: string[] = [];
    for (const [folder, names] of filesByFolder) {
        if (folder === '') continue;
        entries[`${folder}/mod.rs`] = getModPageFragment(names);
        subdirs.push(folder);
    }

    const topLevelMod = topLevelFiles.length > 0 ? getModPageFragment(topLevelFiles) : undefined;
    const subdirsMod = subdirs.length > 0 ? getModPageFragment(subdirs) : undefined;
    entries['mod.rs'] = mergeFragments([topLevelMod, subdirsMod], parts => `${parts.join('\n\n')}\n`);

    return createRenderMap(entries);
}

/**
 * Render a `mod.rs` body that alphabetically declares + re-exports
 * every supplied module name:
 *
 *   mod a;
 *   mod b;
 *
 *   pub use a::*;
 *   pub use b::*;
 */
export function getModPageFragment(names: readonly string[]): Fragment {
    const sorted = [...names].toSorted((a, b) => a.localeCompare(b));
    const modLines = sorted.map(n => `mod ${n};`).join('\n');
    const useLines = sorted.map(n => `pub use ${n}::*;`).join('\n');
    return fragment`${modLines}\n\n${useLines}\n`;
}

/**
 * Group `.rs`-suffixed paths by their parent folder. Top-level files
 * (no slash) land under the `''` key. The `.rs` extension is stripped
 * from each basename so the result feeds directly into
 * {@link getModPageFragment}.
 */
export function groupPathsByFolder(paths: readonly Path[]): Map<string, string[]> {
    const byFolder = new Map<string, string[]>();
    for (const path of paths) {
        const withoutExtension = path.endsWith('.rs') ? path.slice(0, -3) : path;
        // `pathDirectory('AccountNode')` returns `'.'` on Node; normalise
        // to `''` so the top-level sentinel stays consistent.
        const directory = pathDirectory(withoutExtension);
        const folder = directory === '.' ? '' : directory;
        const basename = pathBasename(withoutExtension);
        const names = byFolder.get(folder) ?? [];
        names.push(basename);
        byFolder.set(folder, names);
    }
    return byFolder;
}
