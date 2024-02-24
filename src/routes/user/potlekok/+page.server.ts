import type { PageServerLoad } from './$types';

export const load = (async ({ parent }) => {
	await parent();
}) satisfies PageServerLoad;
