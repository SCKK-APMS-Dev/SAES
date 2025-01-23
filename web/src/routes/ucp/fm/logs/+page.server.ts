import { apiUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

interface Logs {
	owner: string;
	item_id: number | undefined;
	item_type: number | undefined;
	action: string;
	message: string | undefined;
	date: Date;
}

export const load = (async ({ cookies }) => {
	let req = await fetch(`${apiUrl}/ucp/fm/logs/get`, {
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
