import solanaConfig from '@solana-config/oxc/oxlint';
import { defineConfig } from 'oxlint';

export default defineConfig({
    extends: [solanaConfig],
    ignorePatterns: ['**/node_modules/', 'pnpm-lock.yaml'],
    options: { typeAware: true },
    rules: {
        'sort-keys': 'off',
    },
});
