import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	cookies.delete('auth_token', {
		path: '/',
		domain: process.env.NODE_ENV === 'development' ? 'localhost' : 'samt.hu'
	});
	throw redirect(302, '/');
}) satisfies PageServerLoad;
