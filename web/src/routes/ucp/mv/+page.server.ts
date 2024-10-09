import { apiUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies, request }) => {
	const fetcs = await fetch(`${apiUrl}/ucp/mv/home`, {
		headers: {
			cookie: cookies.get('auth_token')!
		}
	});
	console.log(request.headers);
	return {
		stat: await fetcs.json()
	};
}) satisfies PageServerLoad;
