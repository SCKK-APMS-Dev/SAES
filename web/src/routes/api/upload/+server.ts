import { apiUrl } from '$lib/api';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.formData();
	const dcauth = cookies.get('dc-auth') as string;
	if (dcauth) {
		await fetch(`${apiUrl}/user/upload`, {
			method: 'post',
			mode: 'no-cors',
			headers: {
				cookie: dcauth,
				type: request.headers.get('type') as string,
				extra: request.headers.get('extra') as string
			},
			body
		});
	}
	return new Response(body);
};
