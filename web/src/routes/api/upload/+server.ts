import { apiUrl } from '$lib/api';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.formData();
	const dcauth = cookies.get('auth_token') as string;
	if (dcauth) {
		const mama = await fetch(`${apiUrl}/user/items/post`, {
			method: 'post',
			mode: 'no-cors',
			headers: {
				cookie: dcauth
			},
			body
		});
		if (mama.status === 406) {
			return new Response(JSON.stringify({ error: 'toobig' }));
		}
		const bodi = await mama.json();
		return new Response(JSON.stringify(bodi));
	}
	return new Response(body);
};
