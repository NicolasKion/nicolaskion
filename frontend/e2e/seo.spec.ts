// The head, server rendered, against the head Laravel's page had in the
// browser (tests/fixtures/golden/head/home.json).
import { readFileSync } from 'node:fs';
import { expect, test } from '@playwright/test';

import { readHead } from './visual-pages.ts';

const golden = JSON.parse(
  readFileSync(new URL('../../tests/fixtures/golden/head/home.json', import.meta.url), 'utf8'),
);

test('writes the head like Laravel, in the server HTML', async ({ browser, baseURL }) => {
  // Without javascript: the head has to be in the server's HTML.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto('/');
  const actual = await readHead(page);
  // Laravel's url('/') is the request's origin; the golden head was
  // captured from http://127.0.0.1:8314.
  const origin = new URL(baseURL!).origin;
  const rebased = JSON.parse(JSON.stringify(golden).replaceAll('http://127.0.0.1:8314', origin));
  expect(actual).toEqual(rebased);
  await context.close();
});

test('serves robots.txt and no sitemap', async ({ request }) => {
  const robots = await request.get('/robots.txt');
  expect(await robots.text()).toBe('User-agent: *\nDisallow:\n');
  expect((await request.get('/sitemap.xml')).status()).toBe(404);
});

test('preloads the fonts', async ({ page }) => {
  await page.goto('/');
  const preloads = await page
    .locator('link[rel="preload"][as="font"]')
    .evaluateAll((links) => links.map((link) => link.getAttribute('href')));
  expect(preloads).toHaveLength(3);
  for (const href of preloads) {
    expect(href).toMatch(/instrument-sans-(400|500|600)-normal\.[\w-]+\.woff2$/);
  }
});
