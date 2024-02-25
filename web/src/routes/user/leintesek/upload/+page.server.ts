import type { PageServerLoad } from './$types';

export const load = (async ({ parent }) => {
	const par = await parent();
	if (par.layout?.doksi) {
		return {
			db: 0
		};
	}
}) satisfies PageServerLoad;
