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
and the failure is logged. The profile and the repositories fail on their
own. The "GitHub token missing" notice stays away, since a token is set
(the new `githubConfigured` prop tells the two cases apart).

Tests: `golden_pages::github_down_renders_without_github_instead_of_failing`,
`projects_page::a_github_failure_with_nothing_cached_renders_without_github`,
`projects_page::a_failing_profile_keeps_the_repositories`,
`projects-page.svelte.test.ts` "stays away when the token is set but
GitHub failed".

### A failed refresh after the stale window lost the data

Laravel: `Cache::flexible` served stale data only inside its window
(GitHub an hour, ESI a day). After that it fetched before answering, and
a failure then was a 500 for GitHub and an empty dossier for ESI, though
the last good data was at hand.

Now: a failed refresh keeps serving the last good data, inside or past
the stale window, and logs the failure; the next visit tries again.

Tests: `caching::expired_data_is_served_when_the_refresh_fails`,
`caching::a_failed_refresh_keeps_the_stale_data`.

### A malformed answer broke the page for an hour

Laravel: the raw GitHub answers were cached first and turned into objects
on every request, so an answer missing a field was cached and then threw
on each visit until it expired.

Now: an answer is checked before it is cached; one that does not have the
expected shape counts as a failed request.

Test: `caching::a_malformed_answer_is_not_cached`.

### The page scrolled sideways on phones

Laravel: the dot grid behind the header reached 32px past the content on
both sides, so on a 390px wide phone the page was 398px wide and could be
dragged sideways.

Now: the page clips horizontal overflow; the grid looks the same.

Test: `pages.spec.ts` "does not scroll sideways on a phone". The visual
comparison clips the Laravel screenshots to the viewport width.

## Deliberate changes

### The page is rendered on the server

Laravel: no SSR daemon ran, so the HTML had an empty `<div id="app">` and
crawlers saw only the Blade meta tags and the title "Nicolas Kion"; the
browser set "Tools for EVE Online | Nicolas Kion" later.

Now: SvelteKit renders the whole page, with the final title and the same
meta and link tags, in the server's HTML. Relative push dates are worked
out again in the browser.

Tests: `pages.spec.ts` "renders the whole page without javascript",
`seo.spec.ts` "writes the head like Laravel, in the server HTML".

### No cookies

Laravel: every visit set `XSRF-TOKEN` and `nicolas-kion-session`, though
nothing read them.

Now: no cookies, no sessions.

Test: `routes.spec.ts` "sets no cookies".

### Tabs left open on the Laravel site

Laravel: Inertia's client asked for the next page with `X-Inertia: true`.

Now: any request with that header answers 409 with `X-Inertia-Location`
set to the same URL, so the old client reloads into the new page.

Tests: `routes.spec.ts` "sends tabs left open on the Laravel site to a
full reload", `routes::an_inertia_request_reloads_the_page`.

### The `githubConfigured` prop

The api adds `githubConfigured` to the props (normalization N2), so the
page can tell a missing token from a failing GitHub (see the first bug
above).

Test: `golden_pages::github_configured_follows_the_token`.

### ESI requests

Laravel: `https://esi.evetech.net/latest/...` with a trailing slash, no
`X-Compatibility-Date` and Guzzle's User-Agent, the legacy routing CCP
keeps for old clients.

Now: the versionless routes (`/characters/{id}`) with a pinned
`X-Compatibility-Date` and a User-Agent naming the site, as CCP asks.
The answers have the same fields. GitHub requests carry the same
User-Agent; their paths, query strings and headers are unchanged
(normalization N3 maps the ESI paths for the golden comparison).

Tests: `upstream_requests::esi_requests_use_the_compatibility_date`,
`golden_pages::upstream_requests_match_laravel`.

### The update feed is served by Caddy

Laravel: nginx served `/storage/tt-8d20886697466b8297bb6d6e1735fa53/*`
from the `public/storage` symlink; a missing file fell through to
Laravel's local disk route, which answered 403.

Now: Caddy serves the directory straight from the host
(`/opt/nicolaskion/update-feed`), independent of the app containers:
same bytes, `application/json` for `latest.json`, `application/octet-stream`
for the signature and the archives, byte ranges, `Last-Modified` and
conditional requests, no compression. A missing file is a 404, and the
`ETag` values have Caddy's format, so a client holding nginx's ETag gets
one full download.

Tests: `update-feed.spec.ts`, `routing.test.ts`.

### Dropped dead code

Laravel's `inspire` command, the starter kit's `Welcome.vue` (no route
rendered it) and the session, cache and queue tables are gone. Laravel's
local disk routes (`GET` and `PUT /storage/{path}`) only answered signed
URLs and nothing ever signed one; the api keeps answering them with the
same 403 page.

Tests: `golden_http::storage_routes_answer_forbidden`,
`routes::storage_routes_refuse_every_request`.

## Needs owner decision

Kept as Laravel has it, listed for the owner.
