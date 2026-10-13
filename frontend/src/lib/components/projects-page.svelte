<script lang="ts">
  // The landing page: the header with the pilot
  // dossier, the featured work, the searchable repositories and the
  // footer. The root clips horizontal overflow so the dot grid behind the
  // header cannot widen the page on phones.
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
  } from '#lib/projects.ts';
  import type { HomeProps } from '#lib/types.ts';

  let { data }: { data: HomeProps } = $props();

  let search = $state('');
  let activeOwner = $state<string | null>(null);
  // Relative dates are worked out again in the browser once it runs.
  let now = $state(Date.now());
  $effect(() => {
    now = Date.now();
  });

  let name = $derived(displayName(data.profile));
  let githubUrl = $derived(profileUrl(data.profile));
  let ownerCounts = $derived(owners(data.repositories));
  let filteredRepositories = $derived(filterRepositories(data.repositories, search, activeOwner));
  let stars = $derived(totalStars(data.repositories));
  let latest = $derived(latestRepository(data.repositories));

  function toggleOwner(login: string): void {
    activeOwner = activeOwner === login ? null : login;
  }
</script>

<div class="min-h-screen overflow-x-clip bg-paper text-ink dark:bg-void dark:text-neutral-200">
  <div class="mx-auto flex max-w-5xl flex-col gap-20 px-6 py-16 lg:px-8 lg:py-24">
    <header class="relative">
      <div
        aria-hidden="true"
        class="pointer-events-none absolute -inset-x-8 -top-16 -bottom-8 bg-[radial-gradient(circle,rgba(26,26,25,0.22)_1px,transparent_1px)] mask-[radial-gradient(ellipse_70%_90%_at_30%_20%,black,transparent)] bg-size-[26px_26px] dark:bg-[radial-gradient(circle,rgba(231,229,228,0.16)_1px,transparent_1px)]"
      ></div>
      <div
        class="relative flex flex-col gap-12 opacity-100 transition-opacity duration-700 lg:flex-row lg:items-center lg:justify-between starting:opacity-0"
      >
        <div class="flex max-w-xl flex-col gap-8">
          <div class="flex items-center gap-3 font-mono text-xs tracking-widest uppercase">
            <span class="text-accent">//</span>
            <span class="text-neutral-500 dark:text-neutral-400"
              >EVE Online third party developer</span
            >
          </div>
          <h1 class="text-4xl font-semibold tracking-tight text-balance sm:text-6xl">
            Hi, I'm Nicolas.
          </h1>
          <p class="text-lg text-neutral-600 dark:text-neutral-400">
            I play EVE Online and build tools for it. Market helpers, wormhole mapping, killmail
            feeds, settings sync, that kind of thing. Most of it is open source, and all of it lives
            on this page.
          </p>
          <div class="flex flex-wrap gap-3">
            <a
              href={githubUrl}
              target="_blank"
              class="rounded-md bg-ink px-5 py-2 text-sm font-medium text-paper transition-colors hover:bg-black dark:bg-neutral-200 dark:text-void dark:hover:bg-white"
            >
              GitHub
            </a>
            <a
              href="https://www.patreon.com/NicolasKion"
              target="_blank"
              class="rounded-md border border-neutral-300 px-5 py-2 text-sm font-medium transition-colors hover:border-accent hover:text-accent dark:border-neutral-700 dark:hover:border-accent"
            >
              Patreon
            </a>
          </div>
        </div>

        <aside
          class="w-full shrink-0 rounded-lg border border-neutral-200 bg-white font-mono text-xs lg:w-96 dark:border-neutral-800 dark:bg-panel"
        >
          <div
            class="flex items-center gap-3 border-b border-neutral-200 px-5 py-3 tracking-widest uppercase dark:border-neutral-800"
          >
            <span class="text-accent">//</span>
            <span class="text-neutral-500 dark:text-neutral-400">Pilot dossier</span>
            {#if data.character || data.profile}
              <img
                src={data.character?.portraitUrl ?? data.profile?.avatarUrl}
                alt={name}
                class="ml-auto h-8 w-8 rounded-full border border-neutral-300 dark:border-neutral-700"
              />
            {/if}
          </div>
          <dl class="flex flex-col gap-3 px-5 py-4">
            <div class="flex justify-between gap-4">
              <dt class="text-neutral-400 uppercase dark:text-neutral-500">Pilot</dt>
              <dd>{name}</dd>
            </div>
            {#if data.character}
              {@const character = data.character}
              <div class="flex justify-between gap-4">
                <dt class="shrink-0 text-neutral-400 uppercase dark:text-neutral-500">Corp</dt>
                <dd class="truncate">
                  {character.corporationName} [{character.corporationTicker}]
                </dd>
              </div>
              {#if character.allianceName}
                <div class="flex justify-between gap-4">
                  <dt class="shrink-0 text-neutral-400 uppercase dark:text-neutral-500">
                    Alliance
                  </dt>
                  <dd class="truncate">
                    {character.allianceName} [{character.allianceTicker}]
                  </dd>
                </div>
              {/if}
              <div class="flex justify-between gap-4">
                <dt
                  class="shrink-0 whitespace-nowrap text-neutral-400 uppercase dark:text-neutral-500"
                >
                  Sec status
                </dt>
                <dd class={{ 'text-red-500 dark:text-red-400': character.securityStatus < 0 }}>
                  {character.securityStatus.toFixed(1)}
                </dd>
              </div>
              <div class="flex justify-between gap-4">
                <dt
                  class="shrink-0 whitespace-nowrap text-neutral-400 uppercase dark:text-neutral-500"
                >
                  Pilot since
                </dt>
                <dd>{character.pilotSince}</dd>
              </div>
            {/if}
            {#if data.stack.length > 0}
              <div class="flex justify-between gap-4">
                <dt class="text-neutral-400 uppercase dark:text-neutral-500">Stack</dt>
                <dd>{data.stack.join(' · ')}</dd>
              </div>
            {/if}
            {#if data.repositories.length > 0}
              <div class="flex justify-between gap-4">
                <dt class="text-neutral-400 uppercase dark:text-neutral-500">Open source</dt>
                <dd>
                  {data.repositories.length} repos · {stars} stars
                </dd>
              </div>
            {/if}
            {#if latest}
              <div class="flex justify-between gap-4">
                <dt
                  class="shrink-0 whitespace-nowrap text-neutral-400 uppercase dark:text-neutral-500"
                >
                  Last push
                </dt>
                <dd class="flex min-w-0 gap-1.5">
                  <a
                    href={latest.htmlUrl}
                    target="_blank"
                    class="truncate transition-colors hover:text-accent"
                  >
                    {latest.name}
                  </a>
                  <span class="shrink-0 whitespace-nowrap">
                    · {timeAgo(latest.pushedAt, now)}
                  </span>
                </dd>
              </div>
            {/if}
          </dl>
        </aside>
      </div>
    </header>

    <section class="flex flex-col gap-6">
      <div class="flex items-baseline gap-3 font-mono text-xs tracking-widest uppercase">
        <span class="text-accent">//</span>
        <h2 class="text-neutral-600 dark:text-neutral-400">Featured work</h2>
        <span class="text-neutral-400 dark:text-neutral-600">{padCount(data.featured.length)}</span>
        <span class="h-px flex-1 self-center bg-neutral-200 dark:bg-neutral-800"></span>
      </div>
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        {#each data.featured as project (project.name)}
          <article
            class="flex flex-col gap-3 rounded-lg border border-neutral-200 bg-white p-6 transition-colors hover:border-neutral-400 dark:border-neutral-800 dark:bg-panel dark:hover:border-neutral-600"
          >
            <div class="flex min-w-0 items-center gap-3">
              {#if project.logo}
                <img
                  src={project.logo}
                  alt="{project.name} logo"
                  class={[
                    'h-10 w-10 shrink-0 rounded-lg object-contain',
                    { 'dark:hidden': project.logoDark },
                  ]}
                />
              {/if}
              {#if project.logoDark}
                <img
                  src={project.logoDark}
                  alt="{project.name} logo"
                  class="hidden h-10 w-10 shrink-0 rounded-lg object-contain dark:block"
                />
              {/if}
              <h3 class="truncate text-lg font-medium">
                {project.name}
              </h3>
              {#if !project.githubUrl}
                <span
                  class="ml-auto shrink-0 rounded-sm border border-neutral-200 px-1.5 py-0.5 font-mono text-[10px] tracking-wider text-neutral-500 uppercase dark:border-neutral-700 dark:text-neutral-400"
                >
                  Closed source
                </span>
              {/if}
            </div>
            <p class="grow text-sm text-neutral-600 dark:text-neutral-400">
              {project.description}
            </p>
            <div
              class="flex flex-wrap items-center gap-x-4 gap-y-1 font-mono text-[11px] text-neutral-500 dark:text-neutral-400"
            >
              {#if project.language}
                <span class="flex items-center gap-1.5">
                  <span
                    class="h-2 w-2 rounded-full"
                    style:background-color={languageColor(project.language)}
                  ></span>
                  {project.language}
                </span>
              {/if}
              {#if project.stargazersCount}
                <span>★ {project.stargazersCount}</span>
              {/if}
              <span class="ml-auto flex gap-4 tracking-wider uppercase">
                {#if project.siteUrl}
                  <a
                    href={project.siteUrl}
                    target="_blank"
                    class="font-medium text-ink underline-offset-4 transition-colors hover:text-accent hover:underline dark:text-neutral-200"
                  >
                    Visit site
                  </a>
                {/if}
                {#if project.githubUrl}
                  <a
                    href={project.githubUrl}
                    target="_blank"
                    class="font-medium text-ink underline-offset-4 transition-colors hover:text-accent hover:underline dark:text-neutral-200"
                  >
                    Source
                  </a>
                {/if}
              </span>
            </div>
          </article>
        {/each}
      </div>
    </section>

    {#if data.repositories.length > 0}
      <section class="flex flex-col gap-6">
        <div class="flex items-baseline gap-3 font-mono text-xs tracking-widest uppercase">
          <span class="text-accent">//</span>
          <h2 class="text-neutral-600 dark:text-neutral-400">Open source</h2>
          <span class="text-neutral-400 dark:text-neutral-600">
            {padCount(data.repositories.length)} repos / {stars} stars
          </span>
          <span class="h-px flex-1 self-center bg-neutral-200 dark:bg-neutral-800"></span>
        </div>

        <div class="flex flex-col gap-4">
          <input
            bind:value={search}
            type="search"
            placeholder="Search repositories…"
            class="w-full rounded-md border border-neutral-300 bg-white px-4 py-2.5 text-sm transition-colors outline-none placeholder:text-neutral-400 focus:border-accent dark:border-neutral-700 dark:bg-panel dark:placeholder:text-neutral-500"
          />
          <div class="flex flex-wrap gap-2">
            {#each ownerCounts as owner (owner.login)}
              <button
                type="button"
                class={[
                  'flex items-center gap-2 rounded-md border px-3 py-1.5 font-mono text-xs transition-colors',
                  activeOwner === owner.login
                    ? 'border-accent text-accent'
                    : 'border-neutral-300 text-neutral-600 hover:border-neutral-500 dark:border-neutral-700 dark:text-neutral-400 dark:hover:border-neutral-500',
                ]}
                onclick={() => toggleOwner(owner.login)}
              >
                <img src={owner.avatarUrl} alt={owner.login} class="h-4 w-4 rounded-full" />
                {owner.login}
                <span class="opacity-60">{owner.count}</span>
              </button>
            {/each}
          </div>
        </div>

        <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
          {#each filteredRepositories as repository (repository.id)}
            <a
              href={repository.htmlUrl}
              target="_blank"
              class="flex flex-col gap-2.5 rounded-lg border border-neutral-200 bg-white p-5 transition-colors hover:border-neutral-400 dark:border-neutral-800 dark:bg-panel dark:hover:border-neutral-600"
            >
              <div
                class="flex min-w-0 items-center gap-2 font-mono text-[11px] text-neutral-500 dark:text-neutral-400"
              >
                <img
                  src={repository.owner.avatarUrl}
                  alt={repository.owner.login}
                  class="h-4 w-4 shrink-0 rounded-full"
                />
                <span class="truncate">{repository.owner.login}</span>
                {#if repository.isFork}
                  <span
                    class="ml-auto shrink-0 rounded-sm border border-neutral-200 px-1.5 py-0.5 text-[10px] tracking-wider uppercase dark:border-neutral-700"
                  >
                    Fork
                  </span>
                {/if}
                {#if repository.isArchived}
                  <span
                    class={[
                      'shrink-0 rounded-sm border border-amber-400/60 px-1.5 py-0.5 text-[10px] tracking-wider text-amber-700 uppercase dark:border-amber-700 dark:text-amber-400',
                      { 'ml-auto': !repository.isFork },
                    ]}
                  >
                    Archived
                  </span>
                {/if}
              </div>
              <span class="truncate font-medium">{repository.name}</span>
              <p class="line-clamp-2 grow text-sm text-neutral-600 dark:text-neutral-400">
                {repository.description ?? 'No description'}
              </p>
              <div
                class="flex flex-wrap items-center gap-x-3 gap-y-1 font-mono text-[11px] text-neutral-500 dark:text-neutral-400"
              >
                {#if repository.language}
                  <span class="flex items-center gap-1.5">
                    <span
                      class="h-2 w-2 rounded-full"
                      style:background-color={languageColor(repository.language)}
                    ></span>
                    {repository.language}
                  </span>
                {/if}
                {#if repository.stargazersCount > 0}
                  <span>★ {repository.stargazersCount}</span>
                {/if}
                <span class="ml-auto">{timeAgo(repository.pushedAt, now)}</span>
              </div>
            </a>
          {/each}
          {#if filteredRepositories.length === 0}
            <p
              class="col-span-full py-12 text-center text-sm text-neutral-500 dark:text-neutral-400"
            >
              No repositories match your search.
            </p>
          {/if}
        </div>
      </section>
    {/if}

    {#if !data.profile && !data.githubConfigured}
      <div
        class="mx-auto flex max-w-md flex-col gap-2 rounded-lg border border-neutral-200 bg-white p-8 text-center dark:border-neutral-800 dark:bg-panel"
      >
        <h2 class="font-medium">GitHub token missing</h2>
        <p class="text-sm text-neutral-600 dark:text-neutral-400">
          Set <code class="font-mono">GITHUB_TOKEN</code> in your
          <code class="font-mono">.env</code> file to load repositories from GitHub.
        </p>
      </div>
    {/if}

    <p class="text-sm text-neutral-600 dark:text-neutral-400">
      If you like my work, please consider supporting me on
      <a
        href="https://www.patreon.com/NicolasKion"
        target="_blank"
        class="font-medium text-ink underline underline-offset-4 transition-colors hover:text-accent dark:text-neutral-200"
        >Patreon</a
      >.
    </p>

    <footer
      class="flex flex-wrap items-center gap-x-5 gap-y-2 border-t border-neutral-200 pt-6 font-mono text-xs tracking-widest text-neutral-500 uppercase dark:border-neutral-800 dark:text-neutral-400"
    >
      <span class="text-accent">//</span>
      <span>{name}</span>
      <span class="ml-auto flex flex-wrap items-center gap-x-5 gap-y-2">
        <a href={githubUrl} target="_blank" class="transition-colors hover:text-accent"> GitHub </a>
        {#if data.character}
          <a
            href={data.character.zkillboardUrl}
            target="_blank"
            class="transition-colors hover:text-accent"
          >
            zKillboard
          </a>
          <a
            href={data.character.eveWhoUrl}
            target="_blank"
            class="transition-colors hover:text-accent"
          >
            EVE Who
          </a>
        {/if}
        <a
          href="https://www.patreon.com/NicolasKion"
          target="_blank"
          class="transition-colors hover:text-accent"
        >
          Patreon
        </a>
      </span>
    </footer>
  </div>
</div>
