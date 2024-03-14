import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	cookies.delete('sckk-dc-auth', { path: '/' });
	throw redirect(302, '/');
}) satisfies PageServerLoad;
