<script setup lang="ts">
import { Head } from '@inertiajs/vue3';
import { computed, ref } from 'vue';

interface Profile {
    login: string;
    name: string | null;
    avatarUrl: string;
    htmlUrl: string;
}

interface Featured {
    name: string;
    description: string;
    logo: string | null;
    logoDark: string | null;
    siteUrl: string | null;
    githubUrl: string | null;
    language: string | null;
    stargazersCount: number | null;
}

interface Owner {
    login: string;
    avatarUrl: string;
}

interface Repository {
    id: number;
    name: string;
    fullName: string;
    owner: Owner;
    description: string | null;
    htmlUrl: string;
    homepage: string | null;
    language: string | null;
    stargazersCount: number;
    forksCount: number;
    openIssuesCount: number;
    isPrivate: boolean;
    isFork: boolean;
    isArchived: boolean;
    pushedAt: string | null;
    topics: string[];
}

const props = defineProps<{
    profile: Profile | null;
    featured: Featured[];
    repositories: Repository[];
}>();

const search = ref('');
const activeOwner = ref<string | null>(null);

const languageColors: Record<string, string> = {
    PHP: '#4F5D95',
    Vue: '#41b883',
    TypeScript: '#3178c6',
    JavaScript: '#f1e05a',
    Python: '#3572A5',
    Rust: '#dea584',
    Go: '#00ADD8',
    HTML: '#e34c26',
    CSS: '#663399',
    Blade: '#f7523f',
    Shell: '#89e051',
    Dockerfile: '#384d54',
};

const displayName = computed(
    () => props.profile?.name ?? props.profile?.login ?? 'Nicolas Kion',
);
const githubUrl = computed(
    () => props.profile?.htmlUrl ?? 'https://github.com/NicolasKion',
);
const githubHandle = computed(() => props.profile?.login ?? 'NicolasKion');

const owners = computed(() => {
    const counts = new Map<string, Owner & { count: number }>();

    for (const repository of props.repositories) {
        const owner = counts.get(repository.owner.login);

        if (owner) {
            owner.count++;
        } else {
            counts.set(repository.owner.login, {
                ...repository.owner,
                count: 1,
            });
        }
    }

    return [...counts.values()].sort((a, b) => b.count - a.count);
});

const filteredRepositories = computed(() =>
    props.repositories.filter((repository) => {
        if (activeOwner.value && repository.owner.login !== activeOwner.value) {
            return false;
        }

        const query = search.value.trim().toLowerCase();

        if (!query) {
            return true;
        }

        return [
            repository.fullName,
            repository.description ?? '',
            repository.language ?? '',
            ...repository.topics,
        ]
            .join(' ')
            .toLowerCase()
            .includes(query);
    }),
);

const totalStars = computed(() =>
    props.repositories.reduce(
        (sum, repository) => sum + repository.stargazersCount,
        0,
    ),
);

function toggleOwner(login: string): void {
    activeOwner.value = activeOwner.value === login ? null : login;
}

function languageColor(language: string): string {
    return languageColors[language] ?? '#8b949e';
}

function padCount(count: number): string {
    return String(count).padStart(2, '0');
}

function timeAgo(date: string | null): string {
    if (!date) {
        return '';
    }

    const seconds = (Date.now() - new Date(date).getTime()) / 1000;
    const formatter = new Intl.RelativeTimeFormat('en', { numeric: 'auto' });

    const units: [Intl.RelativeTimeFormatUnit, number][] = [
        ['year', 31536000],
        ['month', 2592000],
        ['week', 604800],
        ['day', 86400],
        ['hour', 3600],
        ['minute', 60],
    ];

    for (const [unit, unitSeconds] of units) {
        if (seconds >= unitSeconds) {
            return formatter.format(-Math.round(seconds / unitSeconds), unit);
        }
    }

    return 'just now';
}
</script>

<template>
    <Head title="Tools for EVE Online" />
    <div
        class="min-h-screen bg-paper text-ink dark:bg-void dark:text-neutral-200"
    >
        <div
            class="mx-auto flex max-w-5xl flex-col gap-20 px-6 py-16 lg:px-8 lg:py-24"
        >
            <header class="relative">
                <div
                    aria-hidden="true"
                    class="pointer-events-none absolute -inset-x-8 -top-16 -bottom-8 bg-[radial-gradient(circle,rgba(26,26,25,0.22)_1px,transparent_1px)] [mask-image:radial-gradient(ellipse_70%_90%_at_30%_20%,black,transparent)] bg-[length:26px_26px] dark:bg-[radial-gradient(circle,rgba(231,229,228,0.16)_1px,transparent_1px)]"
                />
                <div
                    class="relative flex flex-col gap-8 opacity-100 transition-opacity duration-700 starting:opacity-0"
                >
                    <div
                        class="flex items-center gap-3 font-mono text-xs tracking-widest uppercase"
                    >
                        <img
                            v-if="profile"
                            :src="profile.avatarUrl"
                            :alt="displayName"
                            class="h-9 w-9 rounded-full border border-neutral-300 dark:border-neutral-700"
                        />
                        <span class="text-accent">//</span>
                        <a
                            :href="githubUrl"
                            target="_blank"
                            class="text-neutral-500 transition-colors hover:text-ink dark:text-neutral-400 dark:hover:text-neutral-200"
                        >
                            {{ displayName }}
                        </a>
                    </div>
                    <h1
                        class="max-w-2xl text-4xl font-semibold tracking-tight text-balance sm:text-6xl"
                    >
                        Hi, I'm Nicolas.
                    </h1>
                    <p
                        class="max-w-xl text-lg text-neutral-600 dark:text-neutral-400"
                    >
                        I play EVE Online and build tools for it. Market
                        helpers, wormhole mapping, killmail feeds, settings
                        sync, that kind of thing. Most of it is open source, and
                        all of it lives on this page.
                    </p>
                    <div class="flex flex-wrap gap-3">
                        <a
                            :href="githubUrl"
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
            </header>

            <section class="flex flex-col gap-6">
                <div
                    class="flex items-baseline gap-3 font-mono text-xs tracking-widest uppercase"
                >
                    <span class="text-accent">//</span>
                    <h2 class="text-neutral-600 dark:text-neutral-400">
                        Featured work
                    </h2>
                    <span class="text-neutral-400 dark:text-neutral-600">{{
                        padCount(featured.length)
                    }}</span>
                    <span
                        class="h-px flex-1 self-center bg-neutral-200 dark:bg-neutral-800"
                    />
                </div>
                <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
                    <article
                        v-for="project in featured"
                        :key="project.name"
                        class="flex flex-col gap-3 rounded-lg border border-neutral-200 bg-white p-6 transition-colors hover:border-neutral-400 dark:border-neutral-800 dark:bg-panel dark:hover:border-neutral-600"
                    >
                        <div class="flex min-w-0 items-center gap-3">
                            <img
                                v-if="project.logo"
                                :src="project.logo"
                                :alt="`${project.name} logo`"
                                class="h-10 w-10 shrink-0 rounded-lg object-contain"
                                :class="{ 'dark:hidden': project.logoDark }"
                            />
                            <img
                                v-if="project.logoDark"
                                :src="project.logoDark"
                                :alt="`${project.name} logo`"
                                class="hidden h-10 w-10 shrink-0 rounded-lg object-contain dark:block"
                            />
                            <h3 class="truncate text-lg font-medium">
                                {{ project.name }}
                            </h3>
                            <span
                                v-if="!project.githubUrl"
                                class="ml-auto shrink-0 rounded-sm border border-neutral-200 px-1.5 py-0.5 font-mono text-[10px] tracking-wider text-neutral-500 uppercase dark:border-neutral-700 dark:text-neutral-400"
                            >
                                Closed source
                            </span>
                        </div>
                        <p
                            class="grow text-sm text-neutral-600 dark:text-neutral-400"
                        >
                            {{ project.description }}
                        </p>
                        <div
                            class="flex flex-wrap items-center gap-x-4 gap-y-1 font-mono text-[11px] text-neutral-500 dark:text-neutral-400"
                        >
                            <span
                                v-if="project.language"
                                class="flex items-center gap-1.5"
                            >
                                <span
                                    class="h-2 w-2 rounded-full"
                                    :style="{
                                        backgroundColor: languageColor(
                                            project.language,
                                        ),
                                    }"
                                />
                                {{ project.language }}
                            </span>
                            <span v-if="project.stargazersCount"
                                >★ {{ project.stargazersCount }}</span
                            >
                            <span
                                class="ml-auto flex gap-4 tracking-wider uppercase"
                            >
                                <a
                                    v-if="project.siteUrl"
                                    :href="project.siteUrl"
                                    target="_blank"
                                    class="font-medium text-ink underline-offset-4 transition-colors hover:text-accent hover:underline dark:text-neutral-200"
                                >
                                    Visit site
                                </a>
                                <a
                                    v-if="project.githubUrl"
                                    :href="project.githubUrl"
                                    target="_blank"
                                    class="font-medium text-ink underline-offset-4 transition-colors hover:text-accent hover:underline dark:text-neutral-200"
                                >
                                    Source
                                </a>
                            </span>
                        </div>
                    </article>
                </div>
            </section>

            <section v-if="repositories.length > 0" class="flex flex-col gap-6">
                <div
                    class="flex items-baseline gap-3 font-mono text-xs tracking-widest uppercase"
                >
                    <span class="text-accent">//</span>
                    <h2 class="text-neutral-600 dark:text-neutral-400">
                        Open source
                    </h2>
                    <span class="text-neutral-400 dark:text-neutral-600">
                        {{ padCount(repositories.length) }} repos /
                        {{ totalStars }} stars
                    </span>
                    <span
                        class="h-px flex-1 self-center bg-neutral-200 dark:bg-neutral-800"
                    />
                </div>

                <div class="flex flex-col gap-4">
                    <input
                        v-model="search"
                        type="search"
                        placeholder="Search repositories…"
                        class="w-full rounded-md border border-neutral-300 bg-white px-4 py-2.5 text-sm transition-colors outline-none placeholder:text-neutral-400 focus:border-accent dark:border-neutral-700 dark:bg-panel dark:placeholder:text-neutral-500"
                    />
                    <div class="flex flex-wrap gap-2">
                        <button
                            v-for="owner in owners"
                            :key="owner.login"
                            type="button"
                            class="flex items-center gap-2 rounded-md border px-3 py-1.5 font-mono text-xs transition-colors"
                            :class="
                                activeOwner === owner.login
                                    ? 'border-accent text-accent'
                                    : 'border-neutral-300 text-neutral-600 hover:border-neutral-500 dark:border-neutral-700 dark:text-neutral-400 dark:hover:border-neutral-500'
                            "
                            @click="toggleOwner(owner.login)"
                        >
                            <img
                                :src="owner.avatarUrl"
                                :alt="owner.login"
                                class="h-4 w-4 rounded-full"
                            />
                            {{ owner.login }}
                            <span class="opacity-60">{{ owner.count }}</span>
                        </button>
                    </div>
                </div>

                <div
                    class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3"
                >
                    <a
                        v-for="repository in filteredRepositories"
                        :key="repository.id"
                        :href="repository.htmlUrl"
                        target="_blank"
                        class="flex flex-col gap-2.5 rounded-lg border border-neutral-200 bg-white p-5 transition-colors hover:border-neutral-400 dark:border-neutral-800 dark:bg-panel dark:hover:border-neutral-600"
                    >
                        <div
                            class="flex min-w-0 items-center gap-2 font-mono text-[11px] text-neutral-500 dark:text-neutral-400"
                        >
                            <img
                                :src="repository.owner.avatarUrl"
                                :alt="repository.owner.login"
                                class="h-4 w-4 shrink-0 rounded-full"
                            />
                            <span class="truncate">{{
                                repository.owner.login
                            }}</span>
                            <span
                                v-if="repository.isFork"
                                class="ml-auto shrink-0 rounded-sm border border-neutral-200 px-1.5 py-0.5 text-[10px] tracking-wider uppercase dark:border-neutral-700"
                            >
                                Fork
                            </span>
                            <span
                                v-if="repository.isArchived"
                                class="shrink-0 rounded-sm border border-amber-400/60 px-1.5 py-0.5 text-[10px] tracking-wider text-amber-700 uppercase dark:border-amber-700 dark:text-amber-400"
                                :class="{ 'ml-auto': !repository.isFork }"
                            >
                                Archived
                            </span>
                        </div>
                        <span class="truncate font-medium">{{
                            repository.name
                        }}</span>
                        <p
                            class="line-clamp-2 grow text-sm text-neutral-600 dark:text-neutral-400"
                        >
                            {{ repository.description ?? 'No description' }}
                        </p>
                        <div
                            class="flex flex-wrap items-center gap-x-3 gap-y-1 font-mono text-[11px] text-neutral-500 dark:text-neutral-400"
                        >
                            <span
                                v-if="repository.language"
                                class="flex items-center gap-1.5"
                            >
                                <span
                                    class="h-2 w-2 rounded-full"
                                    :style="{
                                        backgroundColor: languageColor(
                                            repository.language,
                                        ),
                                    }"
                                />
                                {{ repository.language }}
                            </span>
                            <span v-if="repository.stargazersCount > 0"
                                >★ {{ repository.stargazersCount }}</span
                            >
                            <span class="ml-auto">{{
                                timeAgo(repository.pushedAt)
                            }}</span>
                        </div>
                    </a>
                    <p
                        v-if="filteredRepositories.length === 0"
                        class="col-span-full py-12 text-center text-sm text-neutral-500 dark:text-neutral-400"
                    >
                        No repositories match your search.
                    </p>
                </div>
            </section>

            <div
                v-if="!profile"
                class="mx-auto flex max-w-md flex-col gap-2 rounded-lg border border-neutral-200 bg-white p-8 text-center dark:border-neutral-800 dark:bg-panel"
            >
                <h2 class="font-medium">GitHub token missing</h2>
                <p class="text-sm text-neutral-600 dark:text-neutral-400">
                    Set <code class="font-mono">GITHUB_TOKEN</code> in your
                    <code class="font-mono">.env</code> file to load
                    repositories from GitHub.
                </p>
            </div>

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
                class="flex flex-wrap items-center gap-3 border-t border-neutral-200 pt-6 font-mono text-xs tracking-widest text-neutral-500 uppercase dark:border-neutral-800 dark:text-neutral-400"
            >
                <span class="text-accent">//</span>
                <span>{{ displayName }}</span>
                <a
                    :href="githubUrl"
                    target="_blank"
                    class="ml-auto transition-colors hover:text-accent"
                >
                    github.com/{{ githubHandle }}
                </a>
            </footer>
        </div>
    </div>
</template>
