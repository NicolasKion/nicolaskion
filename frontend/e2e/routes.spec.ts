// The route contract of the whole stack (Caddy, SvelteKit, Axum).
import { expect, test } from '@playwright/test';

test('answers unknown pages with the not found page', async ({ page }) => {
  for (const path of ['/does-not-exist', '/projects', '/login']) {
    const response = await page.goto(path);
    expect(response?.status(), path).toBe(404);
    await expect(page).toHaveTitle('Not Found');
    await expect(page.getByText('404')).toBeVisible();
  }
});

// FIXES.md "Trailing slashes": Laravel answered /up/ like /up.
test('redirects the health page with a trailing slash to the slashless URL', async ({
  request,
}) => {
  const response = await request.get('/up/', { maxRedirects: 0 });
  expect(response.status()).toBe(308);
  expect(new URL(response.headers()['location'], response.url()).pathname).toBe('/up');
});

test('sends the security headers', async ({ request }) => {
  for (const path of [
    '/',
    '/up',
    '/robots.txt',
    '/storage/tt-8d20886697466b8297bb6d6e1735fa53/latest.json',
  ]) {
    const headers = (await request.get(path)).headers();
    expect(headers['x-frame-options'], path).toBe('SAMEORIGIN');
    expect(headers['x-xss-protection'], path).toBe('1; mode=block');
    expect(headers['x-content-type-options'], path).toBe('nosniff');
  }
});

// FIXES.md "No cookies".
test('sets no cookies', async ({ request }) => {
  const response = await request.get('/');
  expect(response.headers()['set-cookie']).toBeUndefined();
});

test('keeps the page out of shared caches', async ({ request }) => {
  const response = await request.get('/');
  expect(response.headers()['cache-control']).toBe('no-cache, private');
});

test('serves the static files', async ({ request }) => {
  for (const path of [
    '/favicon.ico',
    '/icon.png',
    '/apple-touch-icon.png',
    '/og-image.png',
    '/logos/mutamarket.svg',
    '/logos/wormholesystems-light.png',
  ]) {
    expect((await request.get(path)).status(), path).toBe(200);
  }
});

test('answers the health checks', async ({ request }) => {
  const up = await request.get('/up');
  expect(up.status()).toBe(200);
  expect(await up.text()).toContain('Application up');
  const health = await request.get('/api/health');
  expect(await health.json()).toEqual({ status: 'ok' });
});

test('refuses the old storage routes outside the update feed', async ({ request }) => {
  expect((await request.get('/storage/anything.txt')).status()).toBe(403);
  expect((await request.put('/storage/anything.txt', { data: 'x' })).status()).toBe(403);
});

// FIXES.md "Tabs left open on the Laravel site".
test('sends tabs left open on the Laravel site to a full reload', async ({ request, baseURL }) => {
  const response = await request.get('/?from=tab', {
    headers: { 'X-Inertia': 'true', 'X-Inertia-Version': 'old' },
  });
  expect(response.status()).toBe(409);
  expect(response.headers()['x-inertia-location']).toBe(`${baseURL}/?from=tab`);
});
