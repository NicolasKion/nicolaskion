// The page against the screenshots of the Laravel page
// (scripts/laravel-reference/capture.mjs), at desktop and phone width,
// within the tolerance in playwright.config.ts. Both read the same mock
// GitHub and ESI data.
import { expect, test } from '@playwright/test';

import { PAGES, VIEWPORTS, prepare, screenshotOptions } from './visual-pages.ts';

// The baselines were shot on a dev machine; another host rasterizes the
// same fonts a little differently, so CI leaves the comparison to local
// runs (scripts/parity).
test.skip(!!process.env.CI, 'screenshot baselines are host specific');

for (const [name, viewport] of Object.entries(VIEWPORTS)) {
  for (const { slug, path } of PAGES) {
    test(`${slug} looks like the Laravel page (${name})`, async ({ page }) => {
      await page.setViewportSize(viewport);
      await page.goto(path);
      await prepare(page);
      await expect(page).toHaveScreenshot(`${slug}-${name}.png`, await screenshotOptions(page));
    });
  }
}
