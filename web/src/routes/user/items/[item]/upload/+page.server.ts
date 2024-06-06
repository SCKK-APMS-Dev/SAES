import { Reeler_keys, Reeler_vals } from '$lib/public';
import type { PageServerLoad } from './$types';

export const load = (async ({ parent, params }) => {
	await parent();
	if (Reeler_keys.includes(params.item)) {
		const real = Reeler_vals[Reeler_keys.indexOf(params.item)];
		return {
			real
		};
	} else {
		return {
			error: 'Nincs ilyen elem'
		};
	}
}) satisfies PageServerLoad;
