# Fixes

Every Laravel bug the rewrite fixes instead of copying, with the test that
pins the new behavior. Together with the deliberate changes below and the
test normalizations (N1 Inertia's `errors` bag, N2 the new
`githubConfigured` prop, N3 the ESI request paths, all in
`tests/integration/common/golden.rs`), this is the complete list of
differences from the Laravel app on `main`.

## Bugs fixed

### A GitHub failure with nothing cached was a 500

Laravel: `GitHub::profile()` and `repositories()` threw on any HTTP error
and nothing caught it, so a GitHub outage (or a revoked token) with an
empty or expired cache answered the whole page with a 500.

Now: the page renders without the GitHub parts (`profile` null,
`repositories` and `stack` empty, featured projects without source links)
and the failure is logged.

Tests: (C2)

## Deliberate changes

### The page is rendered on the server

Laravel: no SSR daemon ran, so the HTML had an empty `<div id="app">` and
crawlers saw only the Blade meta tags and the title "Nicolas Kion"; the
browser set "Tools for EVE Online | Nicolas Kion" later.

Now: SvelteKit renders the whole page, with the final title and the same
meta tags, in the server's HTML.

Tests: (C2)

## Needs owner decision

Kept as Laravel has it, listed for the owner.
