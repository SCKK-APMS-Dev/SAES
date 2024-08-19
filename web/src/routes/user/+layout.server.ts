import { redirect, type Redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ cookies }) => {
	if (!cookies.get('auth_token')) {
		throw redirect(302, `${apiUrl}/auth`);
	}
	try {
		const aha = await fetch(`${apiUrl}/user`, {
			headers: {
				cookie: cookies.get('auth_token')!
			}
		});
		if (aha.status === 404 || aha.status === 406) {
			throw redirect(
				302,

				`${apiUrl}/auth`
			);
		}
		if (aha.status === 403) {
			return {
				noaccess: true
			};
		}
		if (aha.ok) {
			const jeson = await aha.json();
			return {
				layout: jeson,
				api: apiUrl,
				auth: cookies.get('auth_token')!
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
