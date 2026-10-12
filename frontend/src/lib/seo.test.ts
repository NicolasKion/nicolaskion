import { describe, expect, it } from 'vitest';

import { DESCRIPTION, pageHead } from './seo.ts';

const head = pageHead({ appName: 'Nicolas Kion', origin: 'https://nicolaskion.dev' });

function content(key: string): string | undefined {
  return head.meta.find((tag) => (tag.property ?? tag.name) === key)?.content;
}

describe('pageHead', () => {
  it('titles the page with the app name last', () => {
    expect(head.title).toBe('Tools for EVE Online | Nicolas Kion');
    expect(content('og:title')).toBe('Nicolas Kion | Tools for EVE Online');
    expect(content('twitter:title')).toBe('Nicolas Kion | Tools for EVE Online');
  });

  it('describes the site', () => {
    expect(content('description')).toBe(DESCRIPTION);
    expect(content('og:description')).toBe(DESCRIPTION);
    expect(content('twitter:description')).toBe(DESCRIPTION);
  });

  it('writes OpenGraph tags as properties and the rest as names', () => {
    expect(head.meta.filter((tag) => tag.property).map((tag) => tag.property)).toEqual([
      'og:type',
      'og:site_name',
      'og:title',
      'og:description',
      'og:url',
      'og:image',
      'og:image:width',
      'og:image:height',
      'og:image:alt',
    ]);
    expect(head.meta.filter((tag) => tag.name).map((tag) => tag.name)).toEqual([
      'description',
      'twitter:card',
      'twitter:title',
      'twitter:description',
      'twitter:image',
    ]);
  });

  it('points the canonical URL and the images at the origin, without a trailing slash', () => {
    expect(head.links[0]).toEqual({ rel: 'canonical', href: 'https://nicolaskion.dev' });
    expect(content('og:url')).toBe('https://nicolaskion.dev');
    expect(content('og:image')).toBe('https://nicolaskion.dev/og-image.png');
    expect(content('twitter:image')).toBe('https://nicolaskion.dev/og-image.png');
    expect(content('og:image:alt')).toBe('Nicolas Kion logo');
  });

  it('links the icons', () => {
    expect(head.links.slice(1)).toEqual([
      { rel: 'icon', href: '/favicon.ico', sizes: 'any' },
      { rel: 'icon', href: '/icon.png', type: 'image/png', sizes: '512x512' },
      { rel: 'apple-touch-icon', href: '/apple-touch-icon.png' },
    ]);
  });
});
