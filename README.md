# nicolaskion.dev

My personal landing page at [nicolaskion.dev](https://nicolaskion.dev). It shows the tools I build for the EVE Online community: featured projects like [MutaMarket](https://mutamarket.com), [wormhole.systems](https://wormhole.systems), and [Telescope](https://eve-telescope.com), plus a live list of my public repositories pulled from the GitHub API.

## Stack

- Laravel 13 with PHP 8.4
- Inertia v3 with Vue 3 and TypeScript
- Tailwind CSS v4
- Pest 4, PHPStan (Larastan), Pint, and Rector

## How it works

The home page is a single Inertia page. A `GitHub` service fetches all repositories the configured token can see, caches the raw API responses for ten minutes (stale-while-revalidate up to an hour), and hydrates them into DTOs. Only public, non-fork repositories are shown. Featured projects are curated by hand in `config/projects.php` and only link to their source when the repository is public.

## Local setup

```bash
composer run setup
```

Add a GitHub token to `.env` so the page can load repositories:

```
GITHUB_TOKEN=your-token
```

Then start the dev server with `composer run dev`. Served locally via Laravel Herd at [nicolaskion.test](https://nicolaskion.test).

## Quality checks

```bash
composer run ci:check
```

This runs Pint, Rector, PHPStan, ESLint, Prettier, vue-tsc, and the Pest test suite. The same checks run in GitHub Actions on every push and pull request. Dependabot keeps dependencies fresh weekly; non-major updates merge automatically once CI passes.
