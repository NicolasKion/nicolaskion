// The landing page in the composed stack, with GitHub and ESI answered by
// the mock upstream (tests/fixtures/upstream).
import { expect, test } from '@playwright/test';

import { blockExternalImages, trackJavascriptErrors } from './helpers.ts';

test.beforeEach(async ({ page }) => {
  await blockExternalImages(page);
});

test('renders the dossier, featured work and repositories from the mock data', async ({ page }) => {
  const noErrors = trackJavascriptErrors(page);
  await page.goto('/');
  await expect(page).toHaveTitle('Tools for EVE Online | Nicolas Kion');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText("Hi, I'm Nicolas.");

  const dossier = page.getByRole('complementary');
  await expect(dossier.getByText('Strix Ridens [OWL.]')).toBeVisible();
  await expect(dossier.getByText('Strix Ridens. [STRIX]')).toBeVisible();
  await expect(dossier.getByText('-0.5')).toBeVisible();
  await expect(dossier.getByText('Rust · PHP · TypeScript')).toBeVisible();
  await expect(dossier.getByText('12 repos · 63 stars')).toBeVisible();
  await expect(dossier.getByText(/MutaMarket\s*· 5 years ago/)).toBeVisible();

  await expect(page.getByRole('article')).toHaveCount(8);
  await expect(page.getByText('12 repos / 63 stars')).toBeVisible();
  await expect(page.getByText('GitHub token missing')).toHaveCount(0);
  noErrors();
});

test('searches the repositories and filters them by owner', async ({ page }) => {
  await page.goto('/');
  const cards = page.locator('a[href^="https://github.com/"]').filter({ has: page.locator('p') });
  await expect(cards).toHaveCount(12);

  await page.getByPlaceholder('Search repositories…').fill('laravel');
  await expect(cards).toHaveCount(1);
  await expect(cards.first()).toContainText('loganalyzer');

  await page.getByPlaceholder('Search repositories…').fill('');
  await page.getByRole('button', { name: /WormholeSystems/ }).click();
  await expect(cards).toHaveCount(2);
  await page.getByRole('button', { name: /WormholeSystems/ }).click();
  await expect(cards).toHaveCount(12);

  await page.getByPlaceholder('Search repositories…').fill('no such thing');
  await expect(page.getByText('No repositories match your search.')).toBeVisible();
});

test('opens every outside link in a new tab', async ({ page }) => {
  await page.goto('/');
  const links = page.locator('a[href^="http"]');
  expect(await links.count()).toBeGreaterThan(20);
  for (const link of await links.all()) {
    await expect(link).toHaveAttribute('target', '_blank');
  }
});

// FIXES.md "The page is rendered on the server".
test('renders the whole page without javascript', async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto('/');
  await expect(page).toHaveTitle('Tools for EVE Online | Nicolas Kion');
  await expect(page.getByText('Strix Ridens [OWL.]')).toBeVisible();
  await expect(page.getByRole('article')).toHaveCount(8);
  await expect(page.getByText('loganalyzer', { exact: true })).toBeVisible();
  await context.close();
});

// FIXES.md "The page scrolled sideways on phones".
test('does not scroll sideways on a phone', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  const width = await page.evaluate(() => document.documentElement.scrollWidth);
  expect(width).toBe(390);
});

test('follows the system color scheme', async ({ page }) => {
  await page.emulateMedia({ colorScheme: 'dark' });
  await page.goto('/');
  const dark = await page.evaluate(
    () => getComputedStyle(document.body.firstElementChild!).backgroundColor,
  );
  await page.emulateMedia({ colorScheme: 'light' });
  const light = await page.evaluate(
    () => getComputedStyle(document.body.firstElementChild!).backgroundColor,
  );
  expect(dark).not.toBe(light);
});
