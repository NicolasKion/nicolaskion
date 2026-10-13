// The props of the landing page, as GET /api/pages/home returns them.

export type Profile = {
  login: string;
  name: string | null;
  avatarUrl: string;
  htmlUrl: string;
};

export type Featured = {
  name: string;
  description: string;
  logo: string | null;
  logoDark: string | null;
  siteUrl: string | null;
  githubUrl: string | null;
  language: string | null;
  stargazersCount: number | null;
};

export type Character = {
  id: number;
  name: string;
  corporationName: string;
  corporationTicker: string;
  allianceName: string | null;
  allianceTicker: string | null;
  securityStatus: number;
  pilotSince: number;
  portraitUrl: string;
  zkillboardUrl: string;
  eveWhoUrl: string;
};

export type Owner = {
  login: string;
  avatarUrl: string;
};

export type Repository = {
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
};

export type HomeProps = {
  name: string;
  auth: { user: null };
  /** Whether a GitHub token is set. With one set but GitHub failing and
   * nothing cached, `profile` is null and the GitHub parts are left out
   * without the "token missing" notice. */
  githubConfigured: boolean;
  profile: Profile | null;
  character: Character | null;
  featured: Featured[];
  repositories: Repository[];
  stack: string[];
};
