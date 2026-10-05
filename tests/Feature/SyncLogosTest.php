<?php

use Illuminate\Support\Facades\File;
use Illuminate\Support\Facades\Http;

const PNG_BYTES = "\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0\x01\0\0\0\x01\x08\x06\0\0\0\x1f\x15\xc4\x89";
const SVG_BYTES = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"></svg>';

beforeEach(function (): void {
    $this->publicPath = sys_get_temp_dir().'/logos-sync-'.uniqid();
    File::ensureDirectoryExists($this->publicPath.'/logos');
    app()->usePublicPath($this->publicPath);

    config()->set('projects.featured', [
        [
            'name' => 'Telescope',
            'logo' => '/logos/telescope.png',
            'logo_source' => 'https://raw.githubusercontent.com/eve-telescope/telescope-app/HEAD/icon.png',
        ],
        [
            'name' => 'wormhole.systems',
            'logo' => '/logos/wormholesystems.svg',
            'logo_source' => 'https://raw.githubusercontent.com/WormholeSystems/WormholeSystems/HEAD/logo.svg',
            'logo_dark' => '/logos/wormholesystems-light.svg',
            'logo_dark_source' => 'https://raw.githubusercontent.com/WormholeSystems/WormholeSystems/HEAD/logo-dark.svg',
        ],
    ]);

    Http::preventStrayRequests();
});

afterEach(function (): void {
    File::deleteDirectory($this->publicPath);
});

it('downloads every logo and dark variant from its source', function (): void {
    Http::fake([
        '*/icon.png' => Http::response(PNG_BYTES),
        '*/logo.svg' => Http::response(SVG_BYTES),
        '*/logo-dark.svg' => Http::response(str_replace('1 1', '2 2', SVG_BYTES)),
    ]);

    $this->artisan('logos:sync')->assertSuccessful();

    expect(File::get($this->publicPath.'/logos/telescope.png'))->toBe(PNG_BYTES)
        ->and(File::get($this->publicPath.'/logos/wormholesystems.svg'))->toBe(SVG_BYTES)
        ->and(File::get($this->publicPath.'/logos/wormholesystems-light.svg'))->toContain('2 2');
});

it('keeps the existing logo when the source is missing or not the expected image type', function (int $status, string $body): void {
    File::put($this->publicPath.'/logos/telescope.png', 'existing');

    Http::fake([
        '*/icon.png' => Http::response($body, $status),
        '*.svg' => Http::response(SVG_BYTES),
    ]);

    $this->artisan('logos:sync')->assertFailed();

    expect(File::get($this->publicPath.'/logos/telescope.png'))->toBe('existing')
        ->and($this->publicPath.'/logos/wormholesystems.svg')->toBeFile();
})->with([
    'missing source' => [404, '404: Not Found'],
    'wrong image type' => [200, SVG_BYTES],
]);
