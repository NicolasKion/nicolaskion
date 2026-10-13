import type { Handle, HandleFetch } from '@sveltejs/kit/hooks';
import { AXUM_URL } from '$app/env/private';

// Fonts are preloaded and pages are kept out of shared caches
// (`no-cache, private`).
export const handle: Handle = async ({ event, resolve }) => {
  const response = await resolve(event, {
    preload: ({ type, path }) =>
      type === 'js' || type === 'css' || (type === 'font' && path.endsWith('.woff2')),
  });
  if (!response.headers.has('cache-control')) {
    response.headers.set('cache-control', 'no-cache, private');
  }
  return response;
};

// Server-side loads reach Axum directly; the browser goes through the
// shared-origin proxy instead.
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  const url = new URL(request.url);
  if (url.origin === event.url.origin && url.pathname.startsWith('/api')) {
    request = new Request(new URL(url.pathname + url.search, AXUM_URL), request);
  }

  return fetch(request);
};
