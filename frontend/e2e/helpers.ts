// Shared helpers for the e2e suite.
import { expect, type Page } from '@playwright/test';

/** Collects uncaught page errors and console errors. */
export function trackJavascriptErrors(page: Page): () => void {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(message.text());
    }
  });
  return () => expect(errors, 'javascript errors').toEqual([]);
}

/** Images from GitHub and EVE's image server: the mock data points at
 * avatars that may not exist, so their errors are not the page's. */
export async function blockExternalImages(page: Page): Promise<void> {
  await page.route(/^https:\/\/(avatars\.githubusercontent\.com|images\.evetech\.net)\//, (route) =>
    route.fulfill({ status: 200, contentType: 'image/png', body: Buffer.alloc(0) }),
  );
}
