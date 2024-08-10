import { apiUrl } from '$lib/api';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ request, cookies }) => {
	const dcauth = cookies.get('auth_token') as string;
	if (dcauth) {
		const mama = await fetch(
			`${apiUrl}/user/admin/get?tipus=${request.headers.get('type')}&status=${request.headers.get('status')}`,
			{
				headers: {
					cookie: dcauth
				}
			}
		);
		console.log(mama.status);
		if (mama.ok) {
			return new Response(JSON.stringify({ data: await mama.json(), api: apiUrl }));
		}
	}
	return new Response(null, { status: 400 });
};

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.json();
	if (!body) return new Response(null, { status: 404 });
	const dcauth = cookies.get('auth_token') as string;
	if (dcauth) {
		const mama = await fetch(`${apiUrl}/user/admin/post`, {
			method: 'post',
			headers: {
				cookie: dcauth,
				'Content-Type': 'application/json'
			},
			mode: 'no-cors',
			body: JSON.stringify(body)
		});
		if (mama.ok) {
			return new Response(JSON.stringify(await mama.json()));
		}
		return new Response(JSON.stringify({ error: true }));
	}
	return new Response(null, { status: 400 });
};
