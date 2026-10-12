# Ported tests

Every test of the Laravel app's suite and where it lives now. Rust tests
are `suite::name` in `tests/integration` (one binary, `cargo test`),
vitest tests are `file "name"` under `frontend/src`, Playwright tests are
`file "name"` under `frontend/e2e`.

| PHP test | New test |
| --- | --- |
| `tests/Feature/ExampleTest.php` "returns a successful response" | `projects_page::returns_a_successful_response`; `pages.spec.ts` "renders the dossier, featured work and repositories from the mock data" |
| `tests/Unit/ExampleTest.php` "that true is true" | dropped: asserts nothing |
| `tests/Feature/ProjectsPageTest.php` "renders the landing page with public repositories and featured projects" | `projects_page::renders_the_landing_page_with_public_repositories_and_featured_projects` |
| `ProjectsPageTest` "does not link private repositories in featured projects" | `projects_page::does_not_link_private_repositories_in_featured_projects` |
| `ProjectsPageTest` "renders without github data when no token is configured" | `projects_page::renders_without_github_data_when_no_token_is_configured` (blank tokens too, and no request goes out) |
| `ProjectsPageTest` "shows the eve character in the dossier" | `projects_page::shows_the_eve_character_in_the_dossier` |
| `ProjectsPageTest` "renders without a character when esi is not configured" | `projects_page::renders_without_a_character_when_esi_is_not_configured` |
| `ProjectsPageTest` "fetches repositories across multiple pages" | `projects_page::fetches_repositories_across_multiple_pages` |
| `ProjectsPageTest` "references existing logos with sources and well-formed repositories in the featured config" | `projects_page::references_existing_logos_with_sources_and_well_formed_repositories_in_the_featured_config` |
| `ProjectsPageTest` "links the mutamarket source once the repository is public" | `projects_page::links_the_mutamarket_source_once_the_repository_is_public` |
| `tests/Feature/SyncLogosTest.php` "downloads every logo and dark variant from its source" | `logos::downloads_every_logo_and_dark_variant_from_its_source` |
| `SyncLogosTest` "keeps the existing logo when the source is missing or not the expected image type" (datasets "missing source", "wrong image type") | `logos::keeps_the_existing_logo_when_the_source_is_missing_or_not_the_expected_image_type` (both cases) |

## New test areas

- Golden comparisons with the Laravel reference (`golden_pages`,
  `golden_http`, `golden_php`, `seo.golden.test.ts`, `seo.spec.ts`,
  `visual.spec.ts`): the props of every recorded configuration and
  upstream scenario with their key order, the requests Laravel sent
  upstream, the health and error pages, PHP's rounding, the head and the
  screenshots.
- The upstream clients (`upstream_requests`): headers, query strings,
  paging, retries, timeouts.
- The caches (`caching`): the fresh and stale windows of GitHub and ESI,
  background refreshes, failures with and without cached data, shared
  fetches, the startup warm-up.
- Failure handling (`projects_page`): GitHub and ESI failures, a
  character without alliance, the stack ordering.
- The route contract (`routes`, `harness`, `routes.spec.ts`,
  `routing.test.ts`): health checks, 404 and 403 pages, the Inertia
  reload, security headers, cookies, static files, the Caddyfiles.
- The update feed (`update-feed.spec.ts`): bytes, types, ranges,
  validators.
- The page itself (`projects-page.svelte.test.ts`, `projects.test.ts`,
  `pages.spec.ts`): dossier, featured work, search and owner filter,
  notices, links, rendering without javascript, phone width, dark mode.
