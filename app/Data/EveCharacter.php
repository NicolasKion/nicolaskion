<?php

declare(strict_types=1);

namespace App\Data;

use JsonSerializable;

readonly class EveCharacter implements JsonSerializable
{
    public function __construct(
        public int $id,
        public string $name,
        public string $corporationName,
        public string $corporationTicker,
        public ?string $allianceName,
        public ?string $allianceTicker,
        public float $securityStatus,
        public int $pilotSince,
    ) {}

    public function portraitUrl(): string
    {
        return "https://images.evetech.net/characters/{$this->id}/portrait?size=128";
    }

    public function zkillboardUrl(): string
    {
        return "https://zkillboard.com/character/{$this->id}/";
    }

    public function eveWhoUrl(): string
    {
        return "https://evewho.com/character/{$this->id}";
    }

    /**
     * @return array{id: int, name: string, corporationName: string, corporationTicker: string, allianceName: string|null, allianceTicker: string|null, securityStatus: float, pilotSince: int, portraitUrl: string, zkillboardUrl: string, eveWhoUrl: string}
     */
    public function jsonSerialize(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'corporationName' => $this->corporationName,
            'corporationTicker' => $this->corporationTicker,
            'allianceName' => $this->allianceName,
            'allianceTicker' => $this->allianceTicker,
            'securityStatus' => round($this->securityStatus, 1),
            'pilotSince' => $this->pilotSince,
            'portraitUrl' => $this->portraitUrl(),
            'zkillboardUrl' => $this->zkillboardUrl(),
            'eveWhoUrl' => $this->eveWhoUrl(),
        ];
    }
}
