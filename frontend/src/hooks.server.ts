import type { Handle, HandleFetch } from '@sveltejs/kit/hooks';
import { AXUM_URL } from '$app/env/private';

// Tabs left open on the Laravel site still send Inertia's XHR
// (`X-Inertia: true`); a 409 with `X-Inertia-Location` makes Inertia load
// the same URL in full, which is now this page. Fonts are preloaded like
// the Laravel page did.
export const handle: Handle = async ({ event, resolve }) => {
  if (event.request.headers.has('x-inertia')) {
    return new Response(null, {
      status: 409,
      headers: { 'x-inertia-location': event.url.href },
    });
  }
  return resolve(event, {
    preload: ({ type, path }) =>
      type === 'js' || type === 'css' || (type === 'font' && path.endsWith('.woff2')),
  });
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
