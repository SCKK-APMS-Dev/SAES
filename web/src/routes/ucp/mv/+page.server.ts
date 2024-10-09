import { apiUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	const fetcs = await fetch(`${apiUrl}/ucp/mv/home`, {
		headers: {
			cookie: cookies.get('auth_token')!
		}
	});
	return {
		stat: await fetcs.json()
	};
}) satisfies PageServerLoad;
