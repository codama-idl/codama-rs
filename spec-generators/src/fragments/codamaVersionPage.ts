import { type Fragment, fragment } from '@codama/fragments/rust';

/**
 * The body for `generated/codama_version.rs`: a `CODAMA_VERSION`
 * constant pinned to the spec version at generation time, mirroring
 * the JS `CODAMA_VERSION` constant used by `rootNode()`.
 * Drops the pre-release part (`2.0.0-rc.3` -> `2.0.0`) as JS does: IDLs carry the spec shape, not the package version.
 */
export function getCodamaVersionPageFragment(specVersion: string): Fragment {
    const match = /^(\d+)\.(\d+)\.(\d+)/.exec(specVersion);
    if (!match) throw new Error(`unable to parse a spec version from "${specVersion}".`);
    return fragment`pub const CODAMA_VERSION: &str = "${match[1]}.${match[2]}.${match[3]}";\n`;
}
