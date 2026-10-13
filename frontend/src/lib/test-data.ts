// Props for the frontend tests.
import type { Character, Featured, HomeProps, Repository } from './types.ts';

export function repository(input: {
  id: number;
  owner?: string;
  name?: string;
  description?: string | null;
  language?: string | null;
  stars?: number;
  pushedAt?: string | null;
  topics?: string[];
  isFork?: boolean;
  isArchived?: boolean;
}): Repository {
  const owner = input.owner ?? 'NicolasKion';
  const name = input.name ?? `repo-${input.id}`;
  return {
    id: input.id,
    name,
    fullName: `${owner}/${name}`,
    owner: { login: owner, avatarUrl: `https://avatars.example/${owner}` },
    description: input.description === undefined ? `About ${name}` : input.description,
    htmlUrl: `https://github.com/${owner}/${name}`,
    homepage: null,
    language: input.language === undefined ? 'Rust' : input.language,
    stargazersCount: input.stars ?? 0,
    forksCount: 0,
    openIssuesCount: 0,
    isPrivate: false,
    isFork: input.isFork ?? false,
    isArchived: input.isArchived ?? false,
    pushedAt: input.pushedAt === undefined ? '2021-01-01T00:00:00Z' : input.pushedAt,
    topics: input.topics ?? [],
  };
}

export const character: Character = {
  id: 95569887,
  name: 'Nicolas Kion',
  corporationName: 'Strix Ridens',
  corporationTicker: 'OWL.',
  allianceName: 'Strix Ridens.',
  allianceTicker: 'STRIX',
  securityStatus: -0.5,
  pilotSince: 2015,
  portraitUrl: 'https://images.evetech.net/characters/95569887/portrait?size=128',
  zkillboardUrl: 'https://zkillboard.com/character/95569887/',
  eveWhoUrl: 'https://evewho.com/character/95569887',
};

export const featured: Featured[] = [
  {
    name: 'MutaMarket',
    description: 'The marketplace.',
    logo: '/logos/mutamarket.svg',
    logoDark: null,
    siteUrl: 'https://mutamarket.com',
    githubUrl: 'https://github.com/MutaMarket/MutaMarket',
    language: 'Rust',
    stargazersCount: 12,
  },
  {
    name: 'wormhole.systems',
    description: 'Wormhole mapping.',
    logo: '/logos/wormholesystems.png',
    logoDark: '/logos/wormholesystems-light.png',
    siteUrl: null,
    githubUrl: null,
    language: null,
    stargazersCount: null,
  },
];

export function homeProps(overrides: Partial<HomeProps> = {}): HomeProps {
  return {
    name: 'Nicolas Kion',
    auth: { user: null },
    githubConfigured: true,
    profile: {
      login: 'NicolasKion',
      name: 'Nicolas Kion',
      avatarUrl: 'https://avatars.example/NicolasKion',
      htmlUrl: 'https://github.com/NicolasKion',
    },
    character,
    featured,
    repositories: [
      repository({
        id: 1,
        owner: 'MutaMarket',
        name: 'MutaMarket',
        stars: 12,
        pushedAt: '2021-09-30T18:00:00Z',
        topics: ['eve-online'],
      }),
      repository({
        id: 2,
        owner: 'NicolasKion',
        name: 'KillFeed',
        language: 'Vue',
        stars: 5,
        isArchived: true,
        description: null,
      }),
      repository({
        id: 3,
        owner: 'NicolasKion',
        name: 'dotfiles',
        language: 'Shell',
        pushedAt: null,
      }),
    ],
    stack: ['Rust', 'PHP', 'TypeScript'],
    ...overrides,
  };
}
