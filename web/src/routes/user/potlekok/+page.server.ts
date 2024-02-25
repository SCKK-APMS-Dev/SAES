import { redirect, type Redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ parent, cookies }) => {
	await parent();
	try {
		const aha = await fetch(`${apiUrl}/potlek`, {
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

		if (aha.ok) {
			return {
				potlekok: await aha.json()
			};
		}
	} catch (err) {
		if ((err as Redirect).status) {
			throw redirect((err as Redirect).status, (err as Redirect).location);
		}
		return {
			error: true
		};
	}
}) satisfies PageServerLoad;
