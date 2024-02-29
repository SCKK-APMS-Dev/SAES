import { apiUrl } from '$lib/api';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ request, cookies }) => {
	const dcauth = cookies.get('sckk-dc-auth');
	if (!dcauth) {
		const mama = await fetch(`${apiUrl}/user/admin/get/${request.headers.get('type')}`, {
			headers: {
				cookie: cookies.get('sckk-dc-auth') as string,
				status: request.headers.get('status') as string
			}
		});
		if (mama.ok) {
			return new Response(JSON.stringify(await mama.json()));
		}
	}
	return new Response(null, { status: 400 });
};

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.json();
	if (!body) return new Response(null, { status: 404 });
	const dcauth = cookies.get('sckk-dc-auth');
	if (dcauth) {
		const mama = await fetch(`${apiUrl}/user/admin/post`, {
			method: 'post',
			headers: {
				cookie: cookies.get('sckk-dc-auth') as string,
				'Content-Type': 'application/json'
			},
			mode: 'no-cors',
			body: JSON.stringify(body)
		});
		if (mama.ok) {
			return new Response(JSON.stringify(await mama.json()));
		}
	}
	return new Response(null, { status: 400 });
};
