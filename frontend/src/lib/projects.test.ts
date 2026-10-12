import { describe, expect, it } from 'vitest';

import {
  displayName,
  filterRepositories,
  languageColor,
  latestRepository,
  owners,
  padCount,
  profileUrl,
  timeAgo,
  totalStars,
} from './projects.ts';
import { repository } from './test-data.ts';

const NOW = Date.parse('2026-10-12T12:00:00Z');

describe('languageColor', () => {
  it('uses GitHub colors and grey for the rest', () => {
    expect(languageColor('Rust')).toBe('#dea584');
    expect(languageColor('Vue')).toBe('#41b883');
    expect(languageColor('PHP')).toBe('#4F5D95');
    expect(languageColor('COBOL')).toBe('#8b949e');
  });
});

describe('padCount', () => {
  it('pads to two digits', () => {
    expect(padCount(0)).toBe('00');
    expect(padCount(8)).toBe('08');
    expect(padCount(12)).toBe('12');
    expect(padCount(101)).toBe('101');
  });
});

describe('timeAgo', () => {
  it('names the largest unit that fits, rounded', () => {
    expect(timeAgo('2021-09-30T18:00:00Z', NOW)).toBe('5 years ago');
    expect(timeAgo('2025-10-12T12:00:00Z', NOW)).toBe('last year');
    expect(timeAgo('2026-08-01T00:00:00Z', NOW)).toBe('2 months ago');
    expect(timeAgo('2026-10-01T12:00:00Z', NOW)).toBe('2 weeks ago');
    expect(timeAgo('2026-10-11T12:00:00Z', NOW)).toBe('yesterday');
    expect(timeAgo('2026-10-12T09:00:00Z', NOW)).toBe('3 hours ago');
    expect(timeAgo('2026-10-12T11:59:00Z', NOW)).toBe('1 minute ago');
    expect(timeAgo('2026-10-12T11:59:30Z', NOW)).toBe('just now');
  });

  it('is empty without a date', () => {
    expect(timeAgo(null, NOW)).toBe('');
  });
});

describe('owners', () => {
  it('counts repositories per owner, most first, ties in first-seen order', () => {
    const list = [
      repository({ id: 1, owner: 'b' }),
      repository({ id: 2, owner: 'a' }),
      repository({ id: 3, owner: 'a' }),
      repository({ id: 4, owner: 'c' }),
    ];
    expect(owners(list).map((owner) => [owner.login, owner.count])).toEqual([
      ['a', 2],
      ['b', 1],
      ['c', 1],
    ]);
    expect(owners(list)[0].avatarUrl).toBe('https://avatars.example/a');
  });
});

describe('filterRepositories', () => {
  const list = [
    repository({
      id: 1,
      owner: 'MutaMarket',
      name: 'MutaMarket',
      language: 'Rust',
      topics: ['eve-online'],
    }),
    repository({ id: 2, owner: 'NicolasKion', name: 'KillFeed', description: 'Killmails, LIVE' }),
    repository({
      id: 3,
      owner: 'NicolasKion',
      name: 'dotfiles',
      description: null,
      language: null,
    }),
  ];
  const ids = (found: { id: number }[]) => found.map((item) => item.id);

  it('keeps everything without a search or owner', () => {
    expect(ids(filterRepositories(list, '', null))).toEqual([1, 2, 3]);
    expect(ids(filterRepositories(list, '   ', null))).toEqual([1, 2, 3]);
  });

  it('searches the full name, description, language and topics case-insensitively', () => {
    expect(ids(filterRepositories(list, 'nicolaskion/', null))).toEqual([2, 3]);
    expect(ids(filterRepositories(list, ' live ', null))).toEqual([2]);
    expect(ids(filterRepositories(list, 'rust', null))).toEqual([1]);
    expect(ids(filterRepositories(list, 'EVE-ONLINE', null))).toEqual([1]);
    expect(ids(filterRepositories(list, 'nothing', null))).toEqual([]);
  });

  it('filters by owner, together with the search', () => {
    expect(ids(filterRepositories(list, '', 'NicolasKion'))).toEqual([2, 3]);
    expect(ids(filterRepositories(list, 'dot', 'NicolasKion'))).toEqual([3]);
    expect(ids(filterRepositories(list, 'rust', 'NicolasKion'))).toEqual([]);
  });
});

describe('totals', () => {
  it('sums the stars', () => {
    expect(totalStars([repository({ id: 1, stars: 3 }), repository({ id: 2, stars: 4 })])).toBe(7);
    expect(totalStars([])).toBe(0);
  });

  it('finds the repository pushed to last, unpushed ones last', () => {
    const list = [
      repository({ id: 1, pushedAt: null }),
      repository({ id: 2, pushedAt: '2021-01-01T00:00:00Z' }),
      repository({ id: 3, pushedAt: '2021-06-01T00:00:00Z' }),
    ];
    expect(latestRepository(list)?.id).toBe(3);
    expect(latestRepository([])).toBeNull();
  });
});

describe('profile', () => {
  const profile = {
    login: 'NicolasKion',
    name: 'Nicolas',
    avatarUrl: 'https://avatars.example/n',
    htmlUrl: 'https://github.com/NicolasKion',
  };

  it('names the pilot after the profile, else the login, else Nicolas Kion', () => {
    expect(displayName(profile)).toBe('Nicolas');
    expect(displayName({ ...profile, name: null })).toBe('NicolasKion');
    expect(displayName(null)).toBe('Nicolas Kion');
  });

  it('links the GitHub profile, with a fallback', () => {
    expect(profileUrl(profile)).toBe('https://github.com/NicolasKion');
    expect(profileUrl(null)).toBe('https://github.com/NicolasKion');
  });
});
