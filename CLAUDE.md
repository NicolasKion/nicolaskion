# nicolaskion.dev

Nicolas Kion's landing page: featured EVE Online projects, a pilot
dossier from ESI and his GitHub repositories. A Rust API (Axum, no
database) serving the page props as JSON, and a SvelteKit frontend (SSR,
adapter-node, Svelte 5, Tailwind 4) in `frontend/`. One shared origin
(the Vite dev proxy locally, Caddy in docker and production) routes
backend paths to Axum and everything else to SvelteKit. Never point the
browser at Axum's port directly. The setup mirrors MutaMarket
(`../MutaMarket`) and timkunze.at (`../timkunze`).

## Testing rules

- Every change lands with its tests in the same commit.
- Tests are precise: exact JSON key sets at every level
  (`common::assert_json_eq`), exact messages, statuses and orderings.
- Behavior tests drive the real router (`oneshot`). GitHub and ESI are a
  mock on an ephemeral port (`tests/integration/common/upstream.rs`)
  serving `tests/fixtures/upstream`, never stubs of our own code. The
  caches read a `Clock` the tests move forward.
- Integration suites are modules of the single `tests/integration`
  binary and run single-threaded (`.cargo/config.toml`). Run one suite
  with `cargo test <module>::`.
- Frontend logic has vitest suites next to the code (`*.test.ts`, and
  `*.svelte.test.ts` in a browser). Playwright (`frontend/e2e`) runs
  against the composed stack with `compose.e2e.yml`, which points the
  API at the Python mock in `scripts/upstream-mock.py`.
- Before committing: `cargo fmt`, `cargo clippy --all-targets` (zero
  warnings), `cargo test`, and in `frontend/` `npm run check`,
  `npm run lint`, `npm run format:check`, `npm test`.

## Layout and commands

- `src/github.rs` and `src/esi.rs` the clients, `src/upstream.rs` their
  HTTP layer (timeouts, retries), `src/cache.rs` the stale-while-revalidate
  cache, `src/projects.rs` the page logic (filters, stack, featured),
  `src/featured.rs` the featured projects, `src/server/` the router.
- `frontend/src/lib/components/projects-page.svelte` is the page,
  `frontend/src/lib/projects.ts` its logic, `frontend/src/lib/seo.ts` the
  head. `frontend/proxy-paths.ts` is the routing contract, mirrored by
  `deploy/Caddyfile` (`routing.test.ts` checks). The same Caddyfile runs
  locally and in the production proxy image.
- The update feed under `/storage/tt-8d20886697466b8297bb6d6e1735fa53/`
  is served by Caddy from disk, never by the app. Installed clients have
  the URL compiled in: never move it.
- Whole stack: `docker compose up --build` on http://localhost:5314
  (preview: https://nicolaskion.test).
- `cargo run --bin sync_logos` downloads the featured logos.
- Deploy: merge to main; GitHub Actions builds the api, frontend and
  proxy images, pushes them to GHCR and Coolify redeploys
  (`compose.prod.yml`).
