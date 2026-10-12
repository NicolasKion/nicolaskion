import { error } from '@sveltejs/kit';

/**
 * Loads a JSON payload from the Axum API for an SSR load function; a
 * non-OK status becomes the error page for that status.
 */
export async function apiGet<T>(fetch: typeof globalThis.fetch, path: string): Promise<T> {
  const response = await fetch(path);

  if (!response.ok) {
    const body: { message?: string } = await response.json().catch(() => ({}));
    error(response.status, body.message ?? 'Server Error');
  }

  return response.json();
}
