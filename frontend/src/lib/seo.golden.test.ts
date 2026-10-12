// The head against the one Laravel's page had in the browser
// (tests/fixtures/golden/head/home.json, captured by
// scripts/laravel-reference/capture.mjs from http://127.0.0.1:8314).
import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

import { pageHead } from './seo.ts';

const golden = JSON.parse(
  readFileSync(new URL('../../../tests/fixtures/golden/head/home.json', import.meta.url), 'utf8'),
);

describe('pageHead against Laravel', () => {
  it('has the same title, meta and link tags', () => {
    const head = pageHead({ appName: 'Nicolas Kion', origin: 'http://127.0.0.1:8314' });
    // The viewport tag is static in app.html.
    const meta = golden.meta.filter((tag: Record<string, string>) => tag.name !== 'viewport');
    expect(head.title).toBe(golden.title);
    expect(head.meta).toEqual(meta);
    expect(head.links).toEqual(golden.links);
  });
});
