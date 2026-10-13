// The head in the server HTML, robots.txt and the font preloads.
import { expect, test, type Page } from '@playwright/test';

const DESCRIPTION =
  "Hi, I'm Nicolas. I play EVE Online and build tools for it: MutaMarket, wormhole.systems, Telescope, Killfeed, EVE Wrench, and more.";

/** The SEO-relevant part of <head>: title, meta and link tags. */
async function readHead(page: Page) {
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

test('writes the head in the server HTML', async ({ browser, baseURL }) => {
  // Without javascript: the head has to be in the server's HTML.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto('/');
  const origin = new URL(baseURL!).origin;
  const social = 'Nicolas Kion | Tools for EVE Online';
  expect(await readHead(page)).toEqual({
    title: 'Tools for EVE Online | Nicolas Kion',
    lang: 'en',
    meta: [
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { name: 'description', content: DESCRIPTION },
      { property: 'og:type', content: 'website' },
      { property: 'og:site_name', content: 'Nicolas Kion' },
      { property: 'og:title', content: social },
      { property: 'og:description', content: DESCRIPTION },
      { property: 'og:url', content: origin },
      { property: 'og:image', content: `${origin}/og-image.png` },
      { property: 'og:image:width', content: '1200' },
      { property: 'og:image:height', content: '630' },
      { property: 'og:image:alt', content: 'Nicolas Kion logo' },
      { name: 'twitter:card', content: 'summary_large_image' },
      { name: 'twitter:title', content: social },
      { name: 'twitter:description', content: DESCRIPTION },
      { name: 'twitter:image', content: `${origin}/og-image.png` },
    ],
    links: [
      { rel: 'canonical', href: origin },
      { rel: 'icon', href: '/favicon.ico', sizes: 'any' },
      { rel: 'icon', href: '/icon.png', type: 'image/png', sizes: '512x512' },
      { rel: 'apple-touch-icon', href: '/apple-touch-icon.png' },
    ],
  });
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
