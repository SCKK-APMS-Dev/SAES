import type { PageServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';

export const load = (async ({ cookies }) => {
	// if (cookies.get('auth_token')) {
	// 	return {};
	// }
	// throw redirect(302, 'https://api.ampix.hu/sckk/auth');
	const dcauth = cookies.get('auth_token');
	if (dcauth) {
		throw redirect(302, 'user');
	}
	return {
		noauth: true
	};
}) satisfies PageServerLoad;
