<?php

namespace App\Console\Commands;

use Illuminate\Console\Attributes\Description;
use Illuminate\Console\Attributes\Signature;
use Illuminate\Console\Command;
use Illuminate\Support\Facades\File;
use Illuminate\Support\Facades\Http;

#[Signature('logos:sync')]
#[Description('Download the featured project logos from their source repositories')]
class SyncLogos extends Command
{
    private const array MIME_TYPES = [
        'png' => 'image/png',
        'svg' => 'image/svg+xml',
    ];

    public function handle(): int
    {
        /** @var list<array{name: string, logo: string|null, logo_source?: string, logo_dark?: string|null, logo_dark_source?: string}> $featured */
        $featured = config('projects.featured');

        $logos = collect($featured)
            ->flatMap(fn (array $project): array => [
                [$project['logo'], $project['logo_source'] ?? null],
                [$project['logo_dark'] ?? null, $project['logo_dark_source'] ?? null],
            ])
            ->filter(fn (array $logo): bool => $logo[0] !== null && $logo[1] !== null)
            ->unique(fn (array $logo): string => $logo[0]);

        $failed = false;

        foreach ($logos as [$path, $source]) {
            $error = $this->sync($path, $source);

            if ($error !== null) {
                $failed = true;
                $this->components->error("{$path}: {$error}");

                continue;
            }

            $this->components->info("{$path} synced from {$source}");
        }

        return $failed ? self::FAILURE : self::SUCCESS;
    }

    /**
     * A failed or unexpected download leaves the existing logo untouched.
     */
    private function sync(string $path, string $source): ?string
    {
        $response = Http::timeout(15)->retry([250, 1000], throw: false)->get($source);

        if ($response->failed()) {
            return "download failed with status {$response->status()}";
        }

        $expected = self::MIME_TYPES[pathinfo($path, PATHINFO_EXTENSION)] ?? null;
        $actual = new \finfo(FILEINFO_MIME_TYPE)->buffer($response->body());

        if ($actual !== $expected) {
            return "expected {$expected} but received {$actual}";
        }

        File::put(public_path($path), $response->body());

        return null;
    }
}
