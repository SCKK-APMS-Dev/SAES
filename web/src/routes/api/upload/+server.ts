import { apiUrl } from '$lib/api';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.formData();
	let count = 0;
	console.log('POST START');
	console.log(body);
	body.forEach(() => {
		count++;
	});
	console.log(count);
	const dcauth = cookies.get('auth_token') as string;
	const tipus = request.headers.get('tip');
	console.log(tipus);
	const dates = request.headers.get('dates');
	const ate = JSON.parse(dates!);
	if (tipus != 'leintés' || count % 2 === 0) {
		console.log('POST != LEINTES');
		if (dcauth) {
			console.log('POST = DCAUTH');
			const mama = await fetch(`${apiUrl}/user/items/post?tipus=${tipus}&dates=${ate.toString()}`, {
				method: 'post',
				headers: {
					cookie: dcauth
				},
				body
			});
			console.log(mama.status);
			console.log(mama.statusText);
			if (mama.status === 406) {
				return new Response(JSON.stringify({ error: 'toobig' }));
			}
			const bodi = await mama.json();
			console.log(bodi);
			return new Response(JSON.stringify(bodi));
		}
	} else {
		return new Response(JSON.stringify({ error: 'leintestipik' }));
	}

	return new Response(body);
};
