import { relative } from 'node:path';
import process from 'node:process';

import { generate } from '../src/index';
import { getRepoDirectory } from '../src/repoDirectory';

function main(): void {
    const { outputDir } = generate();
    process.stdout.write(`generated → ${relative(getRepoDirectory(), outputDir)}\n`);
}

try {
    main();
} catch (err: unknown) {
    process.stderr.write(`spec-generators failed: ${err instanceof Error ? err.message : String(err)}\n`);
    if (err instanceof Error && err.stack) process.stderr.write(`${err.stack}\n`);
    process.exit(1);
}
