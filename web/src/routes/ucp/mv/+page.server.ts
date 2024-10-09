import { apiUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies, request }) => {
	const fetcs = await fetch(`${apiUrl}/ucp/mv/home`, {
		headers: {
			cookie: cookies.get('auth_token')!
		}
	});
	return {
		stat: await fetcs.json(),
		country:
			process.env.NODE_ENV === 'development'
				? 'HU'
				: (request.headers.get('cf-ipcountry') as string)
	};
}) satisfies PageServerLoad;
