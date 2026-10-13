// The <head> of the landing page (components/page-head.svelte), as data so
// it can be tested without rendering.

export const DESCRIPTION =
  "Hi, I'm Nicolas. I play EVE Online and build tools for it: MutaMarket, wormhole.systems, Telescope, Killfeed, EVE Wrench, and more.";

/** The page title before the app name. */
export const TITLE = 'Tools for EVE Online';

export type Tag = Record<string, string>;

export type Head = {
  title: string;
  meta: Tag[];
  links: Tag[];
};

/** The head for the site at `origin` (no trailing slash), named
 * `appName`. The canonical URL is the origin itself, without a
 * trailing slash. */
export function pageHead({ appName, origin }: { appName: string; origin: string }): Head {
  const social = `${appName} | ${TITLE}`;
  const image = `${origin}/og-image.png`;
  return {
    title: `${TITLE} | ${appName}`,
    meta: [
      { name: 'description', content: DESCRIPTION },
      { property: 'og:type', content: 'website' },
      { property: 'og:site_name', content: appName },
      { property: 'og:title', content: social },
      { property: 'og:description', content: DESCRIPTION },
      { property: 'og:url', content: origin },
      { property: 'og:image', content: image },
      { property: 'og:image:width', content: '1200' },
      { property: 'og:image:height', content: '630' },
      { property: 'og:image:alt', content: `${appName} logo` },
      { name: 'twitter:card', content: 'summary_large_image' },
      { name: 'twitter:title', content: social },
      { name: 'twitter:description', content: DESCRIPTION },
      { name: 'twitter:image', content: image },
    ],
    links: [
      { rel: 'canonical', href: origin },
      { rel: 'icon', href: '/favicon.ico', sizes: 'any' },
      { rel: 'icon', href: '/icon.png', type: 'image/png', sizes: '512x512' },
      { rel: 'apple-touch-icon', href: '/apple-touch-icon.png' },
    ],
  };
}
