<?php

use Illuminate\Support\Facades\Http;
use Inertia\Testing\AssertableInertia;
use Inertia\Testing\AssertableInertia as Assert;

beforeEach(function (): void {
    config()->set('projects.featured', [
        [
            'name' => 'MutaMarket',
            'description' => 'Closed source marketplace.',
            'logo' => '/logos/mutamarket.svg',
            'site_url' => 'https://mutamarket.com',
            'repository' => null,
        ],
        [
            'name' => 'Killfeed',
            'description' => 'Open source killmail feed.',
            'logo' => '/logos/killfeed.svg',
            'site_url' => null,
            'repository' => 'WormholeSystems/killfeed',
        ],
    ]);
});

function gitHubRepository(array $overrides = []): array
{
    return array_merge([
        'id' => 1,
        'name' => 'killfeed',
        'full_name' => 'WormholeSystems/killfeed',
        'owner' => [
            'login' => 'WormholeSystems',
            'avatar_url' => 'https://avatars.githubusercontent.com/u/1?v=4',
        ],
        'description' => 'A killmail feed',
        'html_url' => 'https://github.com/WormholeSystems/killfeed',
        'homepage' => null,
        'language' => 'Vue',
        'stargazers_count' => 5,
        'forks_count' => 0,
        'open_issues_count' => 0,
        'private' => false,
        'fork' => false,
        'archived' => false,
        'pushed_at' => '2026-07-01T00:00:00Z',
        'topics' => [],
    ], $overrides);
}

it('renders the landing page with public repositories and featured projects', function (): void {
    config()->set('services.github.token', 'fake-token');

    Http::preventStrayRequests();
    Http::fake([
        'api.github.com/user/repos*' => Http::response([
            gitHubRepository(),
            gitHubRepository([
                'id' => 2,
                'name' => 'secret',
                'full_name' => 'NicolasKion/secret',
                'private' => true,
            ]),
            gitHubRepository([
                'id' => 3,
                'name' => 'some-fork',
                'full_name' => 'NicolasKion/some-fork',
                'fork' => true,
            ]),
            gitHubRepository([
                'id' => 4,
                'name' => '.github',
                'full_name' => 'WormholeSystems/.github',
            ]),
        ]),
        'api.github.com/user' => Http::response([
            'login' => 'NicolasKion',
            'name' => 'Nicolas Kion',
            'avatar_url' => 'https://avatars.githubusercontent.com/u/172045106?v=4',
            'html_url' => 'https://github.com/NicolasKion',
        ]),
    ]);

    $this->get('/')
        ->assertOk()
        ->assertInertia(fn (Assert $page): AssertableInertia => $page
            ->component('Projects')
            ->where('profile.login', 'NicolasKion')
            ->has('repositories', 1)
            ->where('repositories.0.fullName', 'WormholeSystems/killfeed')
            ->has('featured', 2)
            ->where('featured.0.name', 'MutaMarket')
            ->where('featured.0.githubUrl', null)
            ->where('featured.1.githubUrl', 'https://github.com/WormholeSystems/killfeed')
            ->where('featured.1.stargazersCount', 5)
        );
});

it('does not link private repositories in featured projects', function (): void {
    config()->set('services.github.token', 'fake-token');

    Http::preventStrayRequests();
    Http::fake([
        'api.github.com/user/repos*' => Http::response([
            gitHubRepository(['private' => true]),
        ]),
        'api.github.com/user' => Http::response([
            'login' => 'NicolasKion',
            'name' => 'Nicolas Kion',
            'avatar_url' => 'https://avatars.githubusercontent.com/u/172045106?v=4',
            'html_url' => 'https://github.com/NicolasKion',
        ]),
    ]);

    $this->get('/')
        ->assertOk()
        ->assertInertia(fn (Assert $page): AssertableInertia => $page
            ->component('Projects')
            ->has('repositories', 0)
            ->where('featured.1.githubUrl', null)
        );
});

it('renders without github data when no token is configured', function (): void {
    config()->set('services.github.token');

    Http::preventStrayRequests();

    $this->get('/')
        ->assertOk()
        ->assertInertia(fn (Assert $page): AssertableInertia => $page
            ->component('Projects')
            ->where('profile', null)
            ->has('repositories', 0)
            ->has('featured', 2)
        );
});

it('shows the eve character in the dossier', function (): void {
    config()->set('services.github.token', 'fake-token');
    config()->set('services.eve.character_id', 95569887);

    Http::preventStrayRequests();
    Http::fake([
        'api.github.com/user/repos*' => Http::response([gitHubRepository()]),
        'api.github.com/user' => Http::response([
            'login' => 'NicolasKion',
            'name' => 'Nicolas Kion',
            'avatar_url' => 'https://avatars.githubusercontent.com/u/172045106?v=4',
            'html_url' => 'https://github.com/NicolasKion',
        ]),
        'esi.evetech.net/latest/characters/*' => Http::response([
            'name' => 'Nicolas Kion',
            'corporation_id' => 98630705,
            'alliance_id' => 99014466,
            'security_status' => -0.5,
            'birthday' => '2015-05-05T14:16:51Z',
        ]),
        'esi.evetech.net/latest/corporations/*' => Http::response([
            'name' => 'Strix Ridens',
            'ticker' => 'OWL.',
        ]),
        'esi.evetech.net/latest/alliances/*' => Http::response([
            'name' => 'Strix Ridens.',
            'ticker' => 'STRIX',
        ]),
    ]);

    $this->get('/')
        ->assertOk()
        ->assertInertia(fn (Assert $page): AssertableInertia => $page
            ->component('Projects')
            ->where('character.name', 'Nicolas Kion')
            ->where('character.corporationTicker', 'OWL.')
            ->where('character.allianceName', 'Strix Ridens.')
            ->where('character.securityStatus', -0.5)
            ->where('character.pilotSince', 2015)
            ->where('character.zkillboardUrl', 'https://zkillboard.com/character/95569887/')
        );
});

it('renders without a character when esi is not configured', function (): void {
    config()->set('services.github.token', 'fake-token');
    config()->set('services.eve.character_id');

    Http::preventStrayRequests();
    Http::fake([
        'api.github.com/user/repos*' => Http::response([gitHubRepository()]),
        'api.github.com/user' => Http::response([
            'login' => 'NicolasKion',
            'name' => 'Nicolas Kion',
            'avatar_url' => 'https://avatars.githubusercontent.com/u/172045106?v=4',
            'html_url' => 'https://github.com/NicolasKion',
        ]),
    ]);

    $this->get('/')
        ->assertOk()
        ->assertInertia(fn (Assert $page): AssertableInertia => $page
            ->component('Projects')
            ->where('character', null)
        );
});

it('fetches repositories across multiple pages', function (): void {
    config()->set('services.github.token', 'fake-token');

    Http::preventStrayRequests();
    Http::fake([
        'api.github.com/user/repos*' => Http::sequence()
            ->push(array_map(
                fn (int $index): array => gitHubRepository(['id' => $index, 'full_name' => "NicolasKion/repo-{$index}"]),
                range(1, 100),
            ))
            ->push([gitHubRepository(['id' => 101, 'full_name' => 'NicolasKion/repo-101'])]),
        'api.github.com/user' => Http::response([
            'login' => 'NicolasKion',
            'name' => 'Nicolas Kion',
            'avatar_url' => 'https://avatars.githubusercontent.com/u/172045106?v=4',
            'html_url' => 'https://github.com/NicolasKion',
        ]),
    ]);

    $this->get('/')
        ->assertOk()
        ->assertInertia(fn (Assert $page): AssertableInertia => $page
            ->component('Projects')
            ->has('repositories', 101)
        );
});
