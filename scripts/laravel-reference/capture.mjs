// Captures what the browser shows of the Laravel reference: the <head>
// (tests/fixtures/golden/head/home.json) and the screenshots Playwright
// compares the new page with
// (frontend/e2e/__screenshots__/visual.spec.ts/*.png).
//
//   scripts/laravel-reference/run.sh start configured
//   node scripts/laravel-reference/capture.mjs
//
// The page list, viewports and masks are shared with
// frontend/e2e/visual-pages.ts so both sides shoot the same thing.

import { mkdirSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  PAGES,
  VIEWPORTS,
  prepare,
  readHead,
  screenshotOptions,
} from '../../frontend/e2e/visual-pages.ts';

const repo = join(dirname(fileURLToPath(import.meta.url)), '../..');
// Playwright comes from the frontend's node_modules.
const { chromium } = createRequire(join(repo, 'frontend/package.json'))('playwright');
const base = process.env.LARAVEL_URL ?? 'http://127.0.0.1:8314';
const headDir = join(repo, 'tests/fixtures/golden/head');
const shotDir = join(repo, 'frontend/e2e/__screenshots__/visual.spec.ts');
mkdirSync(headDir, { recursive: true });
mkdirSync(shotDir, { recursive: true });

const browser = await chromium.launch();
for (const [name, viewport] of Object.entries(VIEWPORTS)) {
  const page = await browser.newPage({ viewport });
  for (const { slug, path } of PAGES) {
    await page.goto(base + path);
    await prepare(page);
    if (name === 'desktop') {
      writeFileSync(
        join(headDir, `${slug}.json`),
        JSON.stringify(await readHead(page), null, 2) + '\n',
      );
    }
    await page.screenshot({
      path: join(shotDir, `${slug}-${name}.png`),
      ...(await screenshotOptions(page)),
    });
    console.log(`${slug} ${name}`);
  }
  await page.close();
}
await browser.close();
