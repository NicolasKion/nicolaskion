# nicolaskion.dev

[![CI](https://github.com/NicolasKion/nicolaskion/actions/workflows/ci.yml/badge.svg)](https://github.com/NicolasKion/nicolaskion/actions/workflows/ci.yml)
[![Website](https://img.shields.io/website?url=https%3A%2F%2Fnicolaskion.dev&label=nicolaskion.dev)](https://nicolaskion.dev)
[![Rust](https://img.shields.io/badge/Rust-Axum-DEA584?logo=rust&logoColor=white)](https://github.com/tokio-rs/axum)
[![SvelteKit](https://img.shields.io/badge/SvelteKit-Svelte%205-FF3E00?logo=svelte&logoColor=white)](https://svelte.dev/docs/kit)

My landing page at [nicolaskion.dev](https://nicolaskion.dev). It shows the
tools I build for the EVE Online community: featured projects like
[MutaMarket](https://mutamarket.com), [wormhole.systems](https://wormhole.systems)
and [Telescope](https://eve-telescope.com), a pilot dossier from EVE's
ESI, and a live, searchable list of my public repositories from the
GitHub API.

## Features

- Featured projects, curated in `src/featured.rs`, linked to their source
  and stats while the repository is public
- Public repositories with search and an owner filter, and the three
  most used languages across everything the token sees (only the
  language names reach the page)
- A pilot dossier from ESI: corporation, alliance, security status
- GitHub and ESI answers cached in memory (GitHub 10 minutes fresh and an
  hour stale, ESI an hour and a day), refreshed in the background. When
  an API fails the page renders without that part, or with the last good
  data
- Server-rendered page with the SEO and social tags, light and dark
  following the system
- The update feed of the tournament-tool desktop app (see below)

## Stack

- API: Rust and [Axum](https://github.com/tokio-rs/axum), no database
- Frontend: [SvelteKit](https://svelte.dev/docs/kit) (SSR, adapter-node),
  Svelte 5 and Tailwind CSS 4
- Caddy in front: `/api/*`, `/storage/*` and `/up` go to the API,
  everything else to SvelteKit. `frontend/proxy-paths.ts` and the two
  Caddyfiles in `deploy/` hold that list.

## Layout

```
src/            the API: GitHub and ESI clients, the cache, the page props, the router
src/bin/        sync_logos, downloads the featured project logos
frontend/       the SvelteKit app and its Playwright suite (frontend/e2e)
tests/          the Rust integration suites and the mock GitHub and ESI data
deploy/         Caddyfiles for the local stack and production
```

## Running it

The whole stack (API, SvelteKit, Caddy) on http://localhost:5314:

```bash
docker compose up --build
```

With the mock GitHub and ESI the end-to-end tests use:

```bash
docker compose -f docker-compose.yml -f compose.e2e.yml up --build -d
```

Or natively:

```bash
cargo run                         # API on 127.0.0.1:3000
cd frontend && npm install && npm run dev
```

Configuration lives in `.env` (see `.env.example`). Without
`GITHUB_TOKEN` the page renders without GitHub data, without
`EVE_CHARACTER_ID` without the dossier. The token stays on the server;
private repositories only count toward the language stack.

## Logos

The featured project logos live in `frontend/static/logos`. Each one has
a source in its project's repository, and

```bash
cargo run --bin sync_logos
```

downloads them again (a failed or unexpected download keeps the current
file). The `logos` workflow runs it monthly and opens a pull request when
a logo changed.

## Update feed

The tournament-tool desktop app checks
`https://nicolaskion.dev/storage/tt-8d20886697466b8297bb6d6e1735fa53/latest.json`
for updates; installed clients have that URL compiled in, so it must keep
working. Caddy serves the directory straight from the host, independent
of the app containers (`deploy/Caddyfile`):

- On the server the files live in `/opt/nicolaskion/update-feed`
  (`latest.json`, `latest.json.sig` and the `update-v*` archives), mounted
  read-only into the shared Caddy container at
  `/srv/nicolaskion/update-feed`.
- tournament-tool's release workflow uploads new releases into that
  directory over rsync, with a key restricted to it.
- Archives and the signature are served as `application/octet-stream`,
  `latest.json` as `application/json`, with byte ranges and conditional
  requests, never compressed.

Locally the composed stack serves `$UPDATE_FEED_DIR` (an empty
`storage/update-feed` by default) the same way.

## Testing

```bash
cargo test
cd frontend && npm run check && npm run lint && npm test
docker compose -f docker-compose.yml -f compose.e2e.yml up --build -d
cd frontend && npm run test:e2e
```

The Rust suites drive the real router against a mock GitHub and ESI on
an ephemeral port. Playwright runs against the composed stack on port
5314.

## Deploying

Production runs on a shared server whose Caddy serves several sites.
`compose.prod.yml` starts only the API and the frontend, from images
tagged with the commit, on the server's external `web` network:

```bash
docker build -t nicolaskion-api:$TAG .
docker build -t nicolaskion-frontend:$TAG frontend
# in /opt/nicolaskion, with IMAGE_TAG=$TAG, GITHUB_TOKEN and EVE_CHARACTER_ID in .env
docker compose -f compose.prod.yml up -d
```

`deploy/Caddyfile` is the production site block, `www.nicolaskion.dev`
included; the server's Caddy keeps a copy of it, so change both.
