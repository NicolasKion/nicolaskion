<?php

namespace App\Http\Controllers;

use App\Data\FeaturedProject;
use App\Data\GitHubRepository;
use App\Services\GitHub;
use Inertia\Inertia;
use Inertia\Response;

class ProjectsController extends Controller
{
    public function __invoke(GitHub $github): Response
    {
        $repositories = $github->isConfigured()
            ? array_values(array_filter(
                $github->repositories(),
                fn (GitHubRepository $repository): bool => ! $repository->isPrivate
                    && ! $repository->isFork
                    && ! str_starts_with($repository->name, '.'),
            ))
            : [];

        return Inertia::render('Projects', [
            'profile' => $github->isConfigured() ? $github->profile() : null,
            'featured' => $this->featuredProjects($repositories),
            'repositories' => $repositories,
        ]);
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
