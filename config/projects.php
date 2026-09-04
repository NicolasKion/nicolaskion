<?php

declare(strict_types=1);

return [

    /*
    |--------------------------------------------------------------------------
    | Featured Projects
    |--------------------------------------------------------------------------
    |
    | Hand-picked projects highlighted on the landing page. The repository
    | key refers to a GitHub "owner/name" and is used to link the source
    | code and show live stats, but only when that repository is public.
    | Logos live in public/logos and are referenced by absolute path.
    |
    */

    'featured' => [
        [
            'name' => 'MutaMarket',
            'description' => 'The marketplace for all your abyssal module needs in EVE Online. Trade, appraise, and browse mutated modules.',
            'logo' => '/logos/mutamarket.svg',
            'site_url' => 'https://mutamarket.com',
            'repository' => 'MutaMarket/MutaMarket',
        ],
        [
            'name' => 'wormhole.systems',
            'description' => 'A collaborative wormhole mapping and tracking tool for EVE Online.',
            'logo' => '/logos/wormholesystems.png',
            'logo_dark' => '/logos/wormholesystems-light.png',
            'site_url' => 'https://wormhole.systems',
            'repository' => 'WormholeSystems/WormholeSystems',
        ],
        [
            'name' => 'EVE Wrench',
            'description' => 'Backup, restore, and sync your EVE Online settings across accounts and characters.',
            'logo' => '/logos/evewrench.png',
            'site_url' => null,
            'repository' => 'eve-wrench/eve-wrench-app',
        ],
        [
            'name' => 'Log Analyzer',
            'description' => 'Analyze your EVE Online combat logs, part of the Telescope toolkit.',
            'logo' => '/logos/loganalyzer.svg',
            'site_url' => 'https://loganalyzer.eve-telescope.com',
            'repository' => 'eve-telescope/loganalyzer',
        ],
        [
            'name' => 'Killfeed',
            'description' => 'A real-time killmail feed for EVE Online at killfeed.killmail.space.',
            'logo' => '/logos/killfeed.svg',
            'site_url' => 'https://killfeed.killmail.space',
            'repository' => 'NicolasKion/KillFeed',
        ],
        [
            'name' => 'Telescope',
            'description' => 'A local scanner tool for EVE Online.',
            'logo' => '/logos/telescope.png',
            'site_url' => 'https://eve-telescope.com',
            'repository' => 'eve-telescope/telescope-app',
        ],
        [
            'name' => 'Marketeer',
            'description' => 'A desktop market tool for EVE Online that monitors your market exports, calculates competitive prices, and copies them to your clipboard.',
            'logo' => '/logos/marketeer.png',
            'site_url' => null,
            'repository' => 'eve-marketeer/marketeer',
        ],
        [
            'name' => 'wormhole.systems Containers',
            'description' => 'A Docker container setup to self-host a private wormhole.systems instance.',
            'logo' => '/logos/wormholesystems.png',
            'logo_dark' => '/logos/wormholesystems-light.png',
            'site_url' => null,
            'repository' => 'WormholeSystems/wormholesystems-containers',
        ],
    ],

];
