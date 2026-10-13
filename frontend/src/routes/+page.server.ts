import { apiGet } from '#lib/server/api.ts';
import type { HomeProps } from '#lib/types.ts';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = ({ fetch }) => apiGet<HomeProps>(fetch, '/api/pages/home');
