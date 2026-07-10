<?php

namespace App\Data;

use JsonSerializable;

readonly class GitHubRepository implements JsonSerializable
{
    /**
     * @param  list<string>  $topics
     */
    public function __construct(
        public int $id,
        public string $name,
        public string $fullName,
        public GitHubOwner $owner,
        public ?string $description,
        public string $htmlUrl,
        public ?string $homepage,
        public ?string $language,
        public int $stargazersCount,
        public int $forksCount,
        public int $openIssuesCount,
        public bool $isPrivate,
        public bool $isFork,
        public bool $isArchived,
        public ?string $pushedAt,
        public array $topics,
    ) {}

    /**
     * @param  array<string, mixed>  $data
     */
    public static function fromResponse(array $data): self
    {
        return new self(
            id: $data['id'],
            name: $data['name'],
            fullName: $data['full_name'],
            owner: GitHubOwner::fromResponse($data['owner']),
            description: $data['description'],
            htmlUrl: $data['html_url'],
            homepage: $data['homepage'],
            language: $data['language'],
            stargazersCount: $data['stargazers_count'],
            forksCount: $data['forks_count'],
            openIssuesCount: $data['open_issues_count'],
            isPrivate: $data['private'],
            isFork: $data['fork'],
            isArchived: $data['archived'],
            pushedAt: $data['pushed_at'],
            topics: $data['topics'] ?? [],
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function jsonSerialize(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'fullName' => $this->fullName,
            'owner' => $this->owner,
            'description' => $this->description,
            'htmlUrl' => $this->htmlUrl,
            'homepage' => $this->homepage,
            'language' => $this->language,
            'stargazersCount' => $this->stargazersCount,
            'forksCount' => $this->forksCount,
            'openIssuesCount' => $this->openIssuesCount,
            'isPrivate' => $this->isPrivate,
            'isFork' => $this->isFork,
            'isArchived' => $this->isArchived,
            'pushedAt' => $this->pushedAt,
            'topics' => $this->topics,
        ];
    }
}
