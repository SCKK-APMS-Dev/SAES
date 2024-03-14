import { redirect, type Redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ cookies }) => {
	try {
		const aha = await fetch(`${apiUrl}/user`, {
			mode: 'no-cors',
			headers: {
				cookie: JSON.stringify(cookies.getAll())
			}
		});
		if (aha.status === 404) {
			throw redirect(
				302,

				`${apiUrl}/user/auth`
			);
		}
		if (aha.status === 401) {
			throw redirect(302, 'noaccess');
		}

		if (aha.ok) {
			return {
				layout: await aha.json()
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
