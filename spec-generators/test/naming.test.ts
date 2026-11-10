import { describe, expect, it } from 'vitest';

import { getRustFieldName } from '../src/naming';

describe('getRustFieldName', () => {
    it('snake-cases the attribute and escapes Rust keywords', () => {
        expect(getRustFieldName('defaultValue')).toBe('default_value');
        expect(getRustFieldName('type')).toBe('r#type');
    });
});
