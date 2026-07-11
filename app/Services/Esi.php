<?php

declare(strict_types=1);

namespace App\Services;

use App\Data\EveCharacter;
use Carbon\CarbonImmutable;
use Illuminate\Http\Client\PendingRequest;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;

class Esi
{
    public function isConfigured(): bool
    {
        return filled(config('services.eve.character_id'));
    }

    public function character(): EveCharacter
    {
        $characterId = (int) config('services.eve.character_id');

        $data = Cache::flexible('esi.character', [3600, 86400], function () use ($characterId): array {
            $character = $this->client()->get("/characters/{$characterId}/")->throw()->json();
            $corporation = $this->client()->get("/corporations/{$character['corporation_id']}/")->throw()->json();

            $alliance = isset($character['alliance_id'])
                ? $this->client()->get("/alliances/{$character['alliance_id']}/")->throw()->json()
                : null;

            return [
                'character' => $character,
                'corporation' => $corporation,
                'alliance' => $alliance,
            ];
        });

        return new EveCharacter(
            id: $characterId,
            name: $data['character']['name'],
            corporationName: $data['corporation']['name'],
            corporationTicker: $data['corporation']['ticker'],
            allianceName: $data['alliance']['name'] ?? null,
            allianceTicker: $data['alliance']['ticker'] ?? null,
            securityStatus: $data['character']['security_status'] ?? 0.0,
            pilotSince: CarbonImmutable::parse($data['character']['birthday'])->year,
        );
    }

    private function client(): PendingRequest
    {
        return Http::baseUrl('https://esi.evetech.net/latest')
            ->accept('application/json')
            ->timeout(10)
            ->connectTimeout(5)
            ->retry([250, 1000]);
    }
}
