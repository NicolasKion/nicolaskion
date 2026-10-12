// The <head> of the landing page (components/page-head.svelte), as data so
// it can be tested without rendering: the tags of Laravel's Blade root
// view, with the title the Inertia page set in the browser.

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
 * `appName`. */
export function pageHead(_input: { appName: string; origin: string }): Head {
  return { title: '', meta: [], links: [] };
}
