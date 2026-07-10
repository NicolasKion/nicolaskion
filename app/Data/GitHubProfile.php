<?php

namespace App\Data;

use JsonSerializable;

readonly class GitHubProfile implements JsonSerializable
{
    public function __construct(
        public string $login,
        public ?string $name,
        public string $avatarUrl,
        public string $htmlUrl,
    ) {}

    /**
     * @param  array<string, mixed>  $data
     */
    public static function fromResponse(array $data): self
    {
        return new self(
            login: $data['login'],
            name: $data['name'],
            avatarUrl: $data['avatar_url'],
            htmlUrl: $data['html_url'],
        );
    }

    /**
     * @return array{login: string, name: string|null, avatarUrl: string, htmlUrl: string}
     */
    public function jsonSerialize(): array
    {
        return [
            'login' => $this->login,
            'name' => $this->name,
            'avatarUrl' => $this->avatarUrl,
            'htmlUrl' => $this->htmlUrl,
        ];
    }
}
