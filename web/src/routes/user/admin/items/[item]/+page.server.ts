import type { PageServerLoad } from './$types';
import { Reeler_keys, Reeler_vals } from '$lib/public';

export const load = (async ({ parent, params, url }) => {
	await parent();
	if (!Reeler_keys.includes(params.item)) return { error: 'Ilyen elem nincs!' };
	const real = Reeler_vals[Reeler_keys.indexOf(params.item)];
	return {
		type: params.item,
		real,
		page: url.searchParams.get('page') ? url.searchParams.get('page') : 0
	};
}) satisfies PageServerLoad;
