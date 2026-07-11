<?php

namespace App\Http\Controllers;

use App\Data\EveCharacter;
use App\Data\FeaturedProject;
use App\Data\GitHubRepository;
use App\Services\Esi;
use App\Services\GitHub;
use Inertia\Inertia;
use Inertia\Response;

class ProjectsController extends Controller
{
    private const array NON_APP_LANGUAGES = ['Dockerfile', 'Shell', 'HTML', 'CSS'];

    public function __invoke(GitHub $github, Esi $esi): Response
    {
        $allRepositories = $github->isConfigured() ? $github->repositories() : [];

        $repositories = array_values(array_filter(
            $allRepositories,
            fn (GitHubRepository $repository): bool => ! $repository->isPrivate
                && ! $repository->isFork
                && ! str_starts_with($repository->name, '.'),
        ));

        return Inertia::render('Projects', [
            'profile' => $github->isConfigured() ? $github->profile() : null,
            'character' => $this->character($esi),
            'featured' => $this->featuredProjects($repositories),
            'repositories' => $repositories,
            'stack' => $this->stackLanguages($allRepositories),
        ]);
    }

    /**
     * Derived from all repositories, including private ones, so the
     * stack reflects what is actually being built. Only language names
     * are exposed to the page.
     *
     * @param  list<GitHubRepository>  $repositories
     * @return list<string>
     */
    private function stackLanguages(array $repositories): array
    {
        $counts = [];

        foreach ($repositories as $repository) {
            $language = $repository->language;
            if ($repository->isFork) {
                continue;
            }
            if ($language === null) {
                continue;
            }
            if (in_array($language, self::NON_APP_LANGUAGES, true)) {
                continue;
            }

            $counts[$language] = ($counts[$language] ?? 0) + 1;
        }

        arsort($counts);

        return array_slice(array_keys($counts), 0, 3);
    }

    /**
     * The page must render even when ESI is down, so failures resolve to null.
     */
    private function character(Esi $esi): ?EveCharacter
    {
        if (! $esi->isConfigured()) {
            return null;
        }

        return rescue(fn (): EveCharacter => $esi->character());
    }

    /**
     * @param  list<GitHubRepository>  $repositories
     * @return list<FeaturedProject>
     */
    private function featuredProjects(array $repositories): array
    {
        $repositoriesByName = collect($repositories)->keyBy(fn (GitHubRepository $repository): string => $repository->fullName);

        /** @var list<array{name: string, description: string, logo: string|null, logo_dark?: string|null, site_url: string|null, repository: string|null}> $featured */
        $featured = config('projects.featured');

        return array_map(fn (array $project): FeaturedProject => new FeaturedProject(
            name: $project['name'],
            description: $project['description'],
            logo: $project['logo'],
            logoDark: $project['logo_dark'] ?? null,
            siteUrl: $project['site_url'],
            repository: $repositoriesByName->get($project['repository'] ?? ''),
        ), $featured);
    }
}
