<?php

namespace App\Services;

use App\Data\GitHubProfile;
use App\Data\GitHubRepository;
use Illuminate\Http\Client\PendingRequest;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;

class GitHub
{
    public function isConfigured(): bool
    {
        return filled(config('services.github.token'));
    }

    public function profile(): GitHubProfile
    {
        return GitHubProfile::fromResponse(
            Cache::flexible('github.profile', [600, 3600], fn (): array => $this->client()->get('/user')->throw()->json())
        );
    }

    /**
     * All repositories the authenticated user owns or can access through an organization.
     *
     * Only raw response arrays are cached so that stale cache entries never
     * hold serialized instances of since-changed classes.
     *
     * @return list<GitHubRepository>
     */
    public function repositories(): array
    {
        $responses = Cache::flexible('github.repositories', [600, 3600], fn (): array => $this->fetchRepositories());

        return array_map(GitHubRepository::fromResponse(...), $responses);
    }

    /**
     * @return list<array<string, mixed>>
     */
    private function fetchRepositories(): array
    {
        $repositories = [];
        $page = 1;

        do {
            $results = $this->client()
                ->get('/user/repos', [
                    'affiliation' => 'owner,organization_member',
                    'sort' => 'pushed',
                    'per_page' => 100,
                    'page' => $page,
                ])
                ->throw()
                ->json();

            array_push($repositories, ...$results);
            $page++;
        } while (count($results) === 100);

        return $repositories;
    }

    private function client(): PendingRequest
    {
        return Http::baseUrl('https://api.github.com')
            ->withToken(config('services.github.token'))
            ->accept('application/vnd.github+json')
            ->timeout(10)
            ->connectTimeout(5)
            ->retry([250, 1000]);
    }
}
