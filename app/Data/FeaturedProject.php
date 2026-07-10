<?php

declare(strict_types=1);

namespace App\Data;

use JsonSerializable;

readonly class FeaturedProject implements JsonSerializable
{
    /**
     * @param  GitHubRepository|null  $repository  The matching public repository, when the source is open.
     */
    /**
     * @param  string|null  $logoDark  Logo variant for dark mode, when the default does not work on dark backgrounds.
     */
    public function __construct(
        public string $name,
        public string $description,
        public ?string $logo,
        public ?string $logoDark,
        public ?string $siteUrl,
        public ?GitHubRepository $repository,
    ) {}

    /**
     * @return array{name: string, description: string, logo: string|null, logoDark: string|null, siteUrl: string|null, githubUrl: string|null, language: string|null, stargazersCount: int|null}
     */
    public function jsonSerialize(): array
    {
        return [
            'name' => $this->name,
            'description' => $this->description,
            'logo' => $this->logo,
            'logoDark' => $this->logoDark,
            'siteUrl' => $this->siteUrl,
            'githubUrl' => $this->repository?->htmlUrl,
            'language' => $this->repository?->language,
            'stargazersCount' => $this->repository?->stargazersCount,
        ];
    }
}
