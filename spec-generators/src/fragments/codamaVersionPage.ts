import { type Fragment, fragment } from '@codama/fragments/rust';

/**
 * The body for `generated/codama_version.rs`: a `CODAMA_VERSION`
 * constant pinned to the spec version at generation time, mirroring
 * the JS `CODAMA_VERSION` constant used by `rootNode()`.
 */
export function getCodamaVersionPageFragment(specVersion: string): Fragment {
    return fragment`pub const CODAMA_VERSION: &str = "${specVersion}";\n`;
}
