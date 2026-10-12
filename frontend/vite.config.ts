import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vitest/config';
import type { ProxyOptions } from 'vite';
import { playwright } from '@vitest/browser-playwright';
import adapter from '@sveltejs/adapter-node';
import { sveltekit } from '@sveltejs/kit/vite';

import { axumPrefixes } from './proxy-paths.ts';

// Dev-only shared origin (Caddy plays this role in docker and production):
// backend paths go to Axum, pages stay in SvelteKit. Never point the
// browser at Axum's port directly. The SSR fetch rewrite in
// hooks.server.ts reads the same variable.
const AXUM_DEV_URL = process.env.AXUM_URL ?? 'http://127.0.0.1:3000';

// File events do not cross Docker's bind mount on every host;
// VITE_POLL_WATCH (set by docker-compose) switches the watcher to polling.
const watch = process.env.VITE_POLL_WATCH ? { usePolling: true, interval: 300 } : undefined;

const proxy: Record<string, ProxyOptions> = {};
for (const prefix of axumPrefixes) {
  proxy[prefix] = { target: AXUM_DEV_URL };
}

export default defineConfig({
  server: { proxy, watch },
  plugins: [
    tailwindcss(),
    sveltekit({
      compilerOptions: {
        // Force runes mode for the project, except for libraries. Can be removed in svelte 6.
        runes: ({ filename }) =>
          filename.split(/[/\\]/).includes('node_modules') ? undefined : true,
      },
      adapter: adapter(),
    }),
  ],
  test: {
    expect: { requireAssertions: true },
    projects: [
      {
        extends: './vite.config.ts',
        test: {
          name: 'client',
          setupFiles: ['./vitest-browser-setup.ts'],
          browser: {
            enabled: true,
            provider: playwright(),
            instances: [{ browser: 'chromium', headless: true }],
          },
          include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
          exclude: ['src/lib/server/**'],
        },
      },
      {
        extends: './vite.config.ts',
        test: {
          name: 'server',
          environment: 'node',
          include: ['src/**/*.{test,spec}.{js,ts}'],
          exclude: ['src/**/*.svelte.{test,spec}.{js,ts}'],
        },
      },
    ],
  },
});
