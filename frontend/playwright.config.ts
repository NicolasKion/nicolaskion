// End-to-end browser tests against the composed stack with the mock
// upstream (`docker compose -f docker-compose.yml -f compose.e2e.yml up
// --build`, the whole app on :5314). Set E2E_BASE_URL to point somewhere
// else, e.g. a vite dev server proxying a local api.
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: 'e2e',
  fullyParallel: false,
  workers: 1,
  retries: 0,
  reporter: [['list']],
  // The visual baselines are the Laravel screenshots
  // (scripts/laravel-reference/capture.mjs).
  snapshotPathTemplate: '{testDir}/__screenshots__/{testFilePath}/{arg}{ext}',
  expect: {
    toHaveScreenshot: { maxDiffPixelRatio: 0.03 },
  },
  use: {
    baseURL: process.env.E2E_BASE_URL ?? 'http://localhost:5314',
    screenshot: 'only-on-failure',
    trace: 'retain-on-failure',
  },
  projects: [{ name: 'chromium', use: { browserName: 'chromium' } }],
});
