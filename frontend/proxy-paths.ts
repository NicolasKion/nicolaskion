// The shared-origin routing contract, mirrored by deploy/Caddyfile.dev and
// deploy/Caddyfile: keep all three in sync when a path changes.

// The tournament-tool update feed. Installed desktop clients have this URL
// compiled in; Caddy serves it straight from a directory on the host, so
// neither the api nor SvelteKit ever sees it.
export const updateFeedPrefix = '/storage/tt-8d20886697466b8297bb6d6e1735fa53/';

// Paths Axum owns outright: every method proxies to the api. `/storage` is
// the rest of Laravel's dead local disk routes (403), `/up` the health
// page.
export const axumPrefixes = ['/api', '/storage', '/up'];
