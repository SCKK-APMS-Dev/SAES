import { apiUrl } from '$lib/api';
import type { Logs } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	let req = await fetch(`${apiUrl}/ucp/admin/faction/logs/get`, {
		headers: {
			cookie: cookies.get('auth_token') as string,
			faction: cookies.get('selected_faction') as string
		}
	});
	let list: Logs[] = await req.json();
	return {
		logs: list
	};
}) satisfies PageServerLoad;
