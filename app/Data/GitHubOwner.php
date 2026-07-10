<?php

namespace App\Data;

use JsonSerializable;

readonly class GitHubOwner implements JsonSerializable
{
    public function __construct(
        public string $login,
        public string $avatarUrl,
    ) {}

    /**
     * @param  array<string, mixed>  $data
     */
    public static function fromResponse(array $data): self
    {
        return new self(
            login: $data['login'],
            avatarUrl: $data['avatar_url'],
        );
    }

    /**
     * @return array{login: string, avatarUrl: string}
     */
    public function jsonSerialize(): array
    {
        return [
            'login' => $this->login,
            'avatarUrl' => $this->avatarUrl,
        ];
    }
}
