import { apiUrl } from '$lib/api';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.formData();
	const dcauth = cookies.get('auth_token') as string;
	const tipus = request.headers.get('tip');
	const dates = request.headers.get('dates');
	const ate = JSON.parse(dates!);
	if (dcauth) {
		const mama = await fetch(`${apiUrl}/user/items/post?tipus=${tipus}&dates=${ate.toString()}`, {
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
