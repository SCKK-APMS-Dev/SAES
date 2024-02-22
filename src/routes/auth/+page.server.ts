import type { PageServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';

import { oauth } from '$lib/server/discord';

export const load = (async () => {
	throw redirect(
		302,
		oauth.generateAuthUrl({
			scope: 'identify'
		})
	);
}) satisfies PageServerLoad;
