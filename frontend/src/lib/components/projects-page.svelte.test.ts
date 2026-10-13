// The landing page rendered on its own, in a browser.
import { describe, expect, it } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';

import ProjectsPage from './projects-page.svelte';
import { character, homeProps } from '../test-data.ts';

describe('the dossier', () => {
  it('shows the pilot, corporation, alliance, security status and stack', async () => {
    await render(ProjectsPage, { data: homeProps() });
    const dossier = page.getByRole('complementary');
    await expect.element(dossier.getByText('Strix Ridens [OWL.]')).toBeVisible();
    await expect.element(dossier.getByText('Strix Ridens. [STRIX]')).toBeVisible();
    await expect.element(dossier.getByText('-0.5')).toHaveClass(/text-red-500/);
    await expect.element(dossier.getByText('2015')).toBeVisible();
    await expect.element(dossier.getByText('Rust · PHP · TypeScript')).toBeVisible();
    await expect.element(dossier.getByText('3 repos · 17 stars')).toBeVisible();
    await expect
      .element(dossier.getByRole('link', { name: 'MutaMarket' }))
      .toHaveAttribute('href', 'https://github.com/MutaMarket/MutaMarket');
    await expect.element(dossier.getByRole('img')).toHaveAttribute('src', character.portraitUrl);
  });

  it('writes whole security statuses with one decimal and not in red', async () => {
    await render(ProjectsPage, {
      data: homeProps({ character: { ...character, securityStatus: 5, allianceName: null } }),
    });
    const dossier = page.getByRole('complementary');
    await expect.element(dossier.getByText('5.0')).not.toHaveClass(/text-red-500/);
    await expect.element(dossier.getByText('Alliance')).not.toBeInTheDocument();
  });

  it('falls back to the GitHub avatar without a character, and to the name without both', async () => {
    const { rerender } = await render(ProjectsPage, { data: homeProps({ character: null }) });
    const dossier = page.getByRole('complementary');
    await expect
      .element(dossier.getByRole('img'))
      .toHaveAttribute('src', 'https://avatars.example/NicolasKion');
    await expect.element(dossier.getByText('Corp')).not.toBeInTheDocument();
    await rerender({
      data: homeProps({ character: null, profile: null, repositories: [], stack: [] }),
    });
    await expect.element(dossier.getByRole('img')).not.toBeInTheDocument();
    await expect.element(dossier.getByText('Nicolas Kion')).toBeVisible();
    await expect.element(dossier.getByText('Stack')).not.toBeInTheDocument();
  });
});

describe('featured work', () => {
  it('lists the projects with their links, stats and logos', async () => {
    await render(ProjectsPage, { data: homeProps() });
    await expect.element(page.getByText('02', { exact: true })).toBeVisible();
    const muta = page.getByRole('article').filter({ hasText: 'MutaMarket' });
    await expect
      .element(muta.getByRole('link', { name: 'Visit site' }))
      .toHaveAttribute('href', 'https://mutamarket.com');
    await expect
      .element(muta.getByRole('link', { name: 'Source' }))
      .toHaveAttribute('href', 'https://github.com/MutaMarket/MutaMarket');
    await expect.element(muta.getByText('★ 12')).toBeVisible();
    await expect.element(muta.getByText('Closed source')).not.toBeInTheDocument();

    const wormhole = page.getByRole('article').filter({ hasText: 'wormhole.systems' });
    await expect.element(wormhole.getByText('Closed source')).toBeVisible();
    await expect.element(wormhole.getByRole('link')).not.toBeInTheDocument();
    // The dark variant is hidden in light mode, so not an accessible image.
    const logos = wormhole.getByAltText('wormhole.systems logo');
    await expect.element(logos.first()).toHaveClass(/dark:hidden/);
    await expect.element(logos.last()).toHaveAttribute('src', '/logos/wormholesystems-light.png');
  });
});

describe('open source', () => {
  it('searches and filters by owner', async () => {
    await render(ProjectsPage, { data: homeProps() });
    const cards = page.getByRole('link').filter({ hasText: /About|No description/ });
    await expect.element(page.getByText('03 repos / 17 stars')).toBeVisible();
    expect(cards.all()).toHaveLength(3);

    await page.getByPlaceholder('Search repositories…').fill('killfeed');
    await expect.poll(() => cards.all().length).toBe(1);
    await page.getByPlaceholder('Search repositories…').fill('');

    const owner = page.getByRole('button', { name: /NicolasKion/ });
    await expect.element(owner).toHaveTextContent('NicolasKion 2');
    await owner.click();
    await expect.poll(() => cards.all().length).toBe(2);
    await expect.element(owner).toHaveClass(/border-accent/);
    await owner.click();
    await expect.poll(() => cards.all().length).toBe(3);

    await page.getByPlaceholder('Search repositories…').fill('nothing at all');
    await expect.element(page.getByText('No repositories match your search.')).toBeVisible();
  });

  it('marks archived repositories and missing descriptions', async () => {
    await render(ProjectsPage, { data: homeProps() });
    const killfeed = page.getByRole('link').filter({ hasText: 'KillFeed' });
    await expect.element(killfeed.getByText('Archived')).toBeVisible();
    await expect.element(killfeed.getByText('No description')).toBeVisible();
    await expect.element(killfeed.getByText('★ 5')).toBeVisible();
  });

  it('is left out without repositories', async () => {
    await render(ProjectsPage, { data: homeProps({ repositories: [] }) });
    await expect.element(page.getByPlaceholder('Search repositories…')).not.toBeInTheDocument();
  });
});

describe('the GitHub notice', () => {
  it('asks for a token when none is configured', async () => {
    await render(ProjectsPage, {
      data: homeProps({ githubConfigured: false, profile: null, repositories: [], stack: [] }),
    });
    await expect.element(page.getByText('GitHub token missing')).toBeVisible();
  });

  it('stays away when the token is set but GitHub failed', async () => {
    await render(ProjectsPage, {
      data: homeProps({ githubConfigured: true, profile: null, repositories: [], stack: [] }),
    });
    await expect.element(page.getByText("Hi, I'm Nicolas.")).toBeVisible();
    await expect.element(page.getByText('GitHub token missing')).not.toBeInTheDocument();
  });

  it('stays away with a profile', async () => {
    await render(ProjectsPage, { data: homeProps() });
    await expect.element(page.getByText("Hi, I'm Nicolas.")).toBeVisible();
    await expect.element(page.getByText('GitHub token missing')).not.toBeInTheDocument();
  });
});

describe('the footer', () => {
  it('links GitHub, the killboards and Patreon', async () => {
    await render(ProjectsPage, { data: homeProps() });
    const footer = page.getByRole('contentinfo');
    await expect
      .element(footer.getByRole('link', { name: 'zKillboard' }))
      .toHaveAttribute('href', character.zkillboardUrl);
    await expect
      .element(footer.getByRole('link', { name: 'EVE Who' }))
      .toHaveAttribute('href', character.eveWhoUrl);
    await expect
      .element(footer.getByRole('link', { name: 'GitHub' }))
      .toHaveAttribute('href', 'https://github.com/NicolasKion');
    await expect
      .element(footer.getByRole('link', { name: 'Patreon' }))
      .toHaveAttribute('href', 'https://www.patreon.com/NicolasKion');
  });

  it('leaves the killboards out without a character', async () => {
    await render(ProjectsPage, { data: homeProps({ character: null }) });
    await expect
      .element(page.getByRole('contentinfo').getByText('zKillboard'))
      .not.toBeInTheDocument();
  });
});
