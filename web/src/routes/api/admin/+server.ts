import { apiUrl } from '$lib/api';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ request, cookies }) => {
	const dcauth = cookies.get('sckk-dc-auth');
	if (dcauth) {
		const mama = await fetch(`${apiUrl}/user/admin/${request.headers.get('type')}`, {
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
