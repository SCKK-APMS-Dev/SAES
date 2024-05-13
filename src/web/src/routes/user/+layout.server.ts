import { redirect, type Redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ cookies }) => {
	try {
		console.log(cookies.get('auth_token'));
		const aha = await fetch(`${apiUrl}/user`, {
			headers: {
				cookie: cookies.get('auth_token')!
			}
		});
		// if (aha.status === 404) {
		// 	throw redirect(
		// 		302,

		// 		`${apiUrl}/auth`
		// 	);
		// }
		if (aha.status === 401) {
			throw redirect(302, '/noaccess');
		}

		if (aha.ok) {
			return {
				layout: await aha.json(),
				api: apiUrl
			};
		}
	} catch (err) {
		if ((err as Redirect).status) {
			throw redirect((err as Redirect).status, (err as Redirect).location);
		}
		return {
			error: 'Weboldal API szerverét nem sikerült elérni'
		};
	}
}) satisfies LayoutServerLoad;
