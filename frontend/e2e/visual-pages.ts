// What the visual comparison shoots, shared by the Laravel capture
// (scripts/laravel-reference/capture.mjs) and visual.spec.ts, so both
// sides render the same page at the same sizes with the same masks. Both
// read GitHub and ESI from the same mock data (tests/fixtures/upstream).
// Plain TypeScript without enums or parameter properties: node runs it
// directly.
import type { Page } from '@playwright/test';

export const PAGES = [{ slug: 'home', path: '/' }];

export const VIEWPORTS = {
  desktop: { width: 1280, height: 800 },
  phone: { width: 390, height: 844 },
};

/** Waits until the page is settled: web fonts loaded, the header's fade
 * in done, nothing left loading. */
export async function prepare(page: Page): Promise<void> {
  await page.waitForLoadState('networkidle');
  await page.evaluate(() => document.fonts.ready);
  await page.waitForTimeout(1000);
}

/** Screenshots cover the whole page at the viewport's width: the Laravel
 * page is 8px wider than a phone (FIXES.md "The page scrolled sideways on
 * phones"), which is clipped off. Images from GitHub and EVE's image
 * server are masked: the mock data points at avatars that may or may not
 * load. */
export async function screenshotOptions(page: Page) {
  const width = page.viewportSize()?.width ?? 0;
  const height = await page.evaluate(() => document.documentElement.scrollHeight);
  return {
    fullPage: true,
    clip: { x: 0, y: 0, width, height },
    animations: 'disabled' as const,
    caret: 'hide' as const,
    mask: [page.locator('img[src^="https://"]')],
  };
}

/** The SEO-relevant part of <head>: title, meta and link tags. */
export async function readHead(page: Page) {
  return page.evaluate(() => {
    // Event handler attributes (Svelte's onload replay on <link>) are not
    // part of the head's content.
    const attributes = (element: Element) =>
      Object.fromEntries(
        [...element.attributes]
          .filter((a) => !a.name.startsWith('on'))
          .map((a) => [a.name, a.value]),
      );
    return {
      title: document.title,
      lang: document.documentElement.lang,
      meta: [...document.head.querySelectorAll('meta')]
        .map(attributes)
        .filter((meta) => meta.name || meta.property),
      links: [...document.head.querySelectorAll('link')]
        .map(attributes)
        .filter((link) => ['canonical', 'icon', 'apple-touch-icon'].includes(link.rel)),
    };
  });
}
