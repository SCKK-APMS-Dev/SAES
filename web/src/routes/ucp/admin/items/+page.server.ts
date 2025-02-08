import { apiUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	if (!cookies.get('auth_token')) {
		return {
			noauth: true
		};
	}
	if (!cookies.get('selected_faction')) return {};
	const fetcs = await fetch(`${apiUrl}/ucp/admin/items/home`, {
		headers: {
			cookie: cookies.get('auth_token')!,
			faction: cookies.get('selected_faction')!
		}
	});
	return {
		stat: await fetcs.json()
	};
}) satisfies PageServerLoad;
