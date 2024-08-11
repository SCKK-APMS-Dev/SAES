import type { PageServerLoad } from './$types';

export const load = (async ({ parent, url }) => {
	await parent();
	return {
		page: url.searchParams.get('page') ? url.searchParams.get('page') : 0,
		status: url.searchParams.get('status') ? url.searchParams.get('status') : 'feltöltve'
	};
}) satisfies PageServerLoad;
