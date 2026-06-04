import { describe, expect, it } from 'vitest';

import { CATEGORY_DIRECTORIES } from '../src/defaults';
import { buildRenderScope, type RenderOptions } from '../src/options';

const options: RenderOptions = { targetSpecMajor: 1 };

describe('buildRenderScope', () => {
    it('defaults categoryDirectories to the v1 table', () => {
        const scope = buildRenderScope(options);
        expect(scope.categoryDirectories).toBe(CATEGORY_DIRECTORIES);
    });

    it('threads the caller-supplied targetSpecMajor through', () => {
        const scope = buildRenderScope({ targetSpecMajor: 2 });
        expect(scope.targetSpecMajor).toBe(2);
    });

    it('returns a frozen scope', () => {
        const scope = buildRenderScope(options);
        expect(Object.isFrozen(scope)).toBe(true);
    });

    it('honours a caller-supplied categoryDirectories override', () => {
        const override = new Map([['link', 'custom_link_dir']]);
        const scope = buildRenderScope({ ...options, categoryDirectories: override });
        expect(scope.categoryDirectories).toBe(override);
    });
});
