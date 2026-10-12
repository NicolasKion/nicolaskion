// The landing page's logic, ported from Projects.vue: language colors,
// the owner filter, the repository search and the relative push dates.
import type { Owner, Profile, Repository } from './types.ts';

export type OwnerCount = Owner & { count: number };

/** GitHub's color for a language, grey for the rest. */
export function languageColor(_language: string): string {
  return '';
}

/** Two digits at least: 8 is "08". */
export function padCount(_count: number): string {
  return '';
}

/** "5 years ago", "yesterday", "just now", relative to `now` (ms). */
export function timeAgo(_date: string | null, _now: number): string {
  return '';
}

/** The owners of the repositories, most repositories first. */
export function owners(_repositories: Repository[]): OwnerCount[] {
  return [];
}

/** The repositories of `owner` (all when null) matching `search` in their
 * full name, description, language or topics, case-insensitively. */
export function filterRepositories(
  _repositories: Repository[],
  _search: string,
  _owner: string | null,
): Repository[] {
  return [];
}

export function totalStars(_repositories: Repository[]): number {
  return -1;
}

/** The repository pushed to last. */
export function latestRepository(_repositories: Repository[]): Repository | null {
  return null;
}

/** The name in the dossier and footer. */
export function displayName(_profile: Profile | null): string {
  return '';
}

/** Where the GitHub buttons lead. */
export function profileUrl(_profile: Profile | null): string {
  return '';
}
