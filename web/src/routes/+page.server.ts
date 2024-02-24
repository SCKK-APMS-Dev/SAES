import type { PageServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';

export const load = (async ({ cookies }) => {
	// if (cookies.get('dc-auth')) {
	// 	return {};
	// }
	// throw redirect(302, 'https://api.ampix.hu/sckk/auth');
	const dcauth = cookies.get('sckk-dc-auth');
	if (dcauth) {
		throw redirect(302, 'user');
	}
	return {
		noauth: true
	};
}) satisfies PageServerLoad;
