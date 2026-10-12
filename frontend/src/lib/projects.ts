// The landing page's logic, ported from Projects.vue: language colors,
// the owner filter, the repository search and the relative push dates.
import type { Owner, Profile, Repository } from './types.ts';

export type OwnerCount = Owner & { count: number };

const LANGUAGE_COLORS: Record<string, string> = {
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

/** GitHub's color for a language, grey for the rest. */
export function languageColor(language: string): string {
  return LANGUAGE_COLORS[language] ?? '#8b949e';
}

/** Two digits at least: 8 is "08". */
export function padCount(count: number): string {
  return String(count).padStart(2, '0');
}

const UNITS: [Intl.RelativeTimeFormatUnit, number][] = [
  ['year', 31536000],
  ['month', 2592000],
  ['week', 604800],
  ['day', 86400],
  ['hour', 3600],
  ['minute', 60],
];

/** "5 years ago", "yesterday", "just now", relative to `now` (ms). */
export function timeAgo(date: string | null, now: number): string {
  if (!date) {
    return '';
  }
  const seconds = (now - new Date(date).getTime()) / 1000;
  const formatter = new Intl.RelativeTimeFormat('en', { numeric: 'auto' });
  for (const [unit, unitSeconds] of UNITS) {
    if (seconds >= unitSeconds) {
      return formatter.format(-Math.round(seconds / unitSeconds), unit);
    }
  }
  return 'just now';
}

/** The owners of the repositories, most repositories first. */
export function owners(repositories: Repository[]): OwnerCount[] {
  const counts = new Map<string, OwnerCount>();
  for (const repository of repositories) {
    const owner = counts.get(repository.owner.login);
    if (owner) {
      owner.count++;
    } else {
      counts.set(repository.owner.login, { ...repository.owner, count: 1 });
    }
  }
  return [...counts.values()].sort((a, b) => b.count - a.count);
}

/** The repositories of `owner` (all when null) matching `search` in their
 * full name, description, language or topics, case-insensitively. */
export function filterRepositories(
  repositories: Repository[],
  search: string,
  owner: string | null,
): Repository[] {
  const query = search.trim().toLowerCase();
  return repositories.filter((repository) => {
    if (owner && repository.owner.login !== owner) {
      return false;
    }
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
  });
}

export function totalStars(repositories: Repository[]): number {
  return repositories.reduce((sum, repository) => sum + repository.stargazersCount, 0);
}

/** The repository pushed to last. */
export function latestRepository(repositories: Repository[]): Repository | null {
  return (
    [...repositories].sort((a, b) => (b.pushedAt ?? '').localeCompare(a.pushedAt ?? ''))[0] ?? null
  );
}

/** The name in the dossier and footer. */
export function displayName(profile: Profile | null): string {
  return profile?.name ?? profile?.login ?? 'Nicolas Kion';
}

/** Where the GitHub buttons lead. */
export function profileUrl(profile: Profile | null): string {
  return profile?.htmlUrl ?? 'https://github.com/NicolasKion';
}
