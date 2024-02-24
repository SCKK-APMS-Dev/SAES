import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	try {
		const aha = await fetch('http://localhost:3000/user', {
			mode: 'no-cors',
			headers: {
				cookie: JSON.stringify(cookies.getAll())
			}
		});
		if (aha.status === 404) {
			throw redirect(
				302,
				process.env.NODE_ENV === 'production'
					? 'https://sckk-api.ampix.hu/user/auth'
					: 'http://localhost:3000/user/auth'
			);
		}
		if (aha.status === 401) {
			throw redirect(302, 'noaccess');
		}
		if (aha.status === 200) {
			return {
				layout: {
					doksi: await aha.json()
				}
			};
		}
	} catch {
		return {
			error: true
		};
	}
}) satisfies LayoutServerLoad;
