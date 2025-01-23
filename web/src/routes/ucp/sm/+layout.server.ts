import { isRedirect, redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { apiUrl, apiUrlPublic } from '$lib/api';

export const load = (async ({ cookies }) => {
	if (!cookies.get('auth_token')) {
		return {
			noauth: true,
			api: apiUrlPublic
		};
	}
	try {
		const aha = await fetch(`${apiUrl}/ucp/sm`, {
			mode: 'no-cors',
			headers: {
				cookie: cookies.get('auth_token') as string,
				faction: cookies.get('selected_faction') as string
			}
		});
		if (aha.status === 404) {
			return {
				noauth: true,
				api: apiUrlPublic
			};
		}
		if (aha.status === 403) {
			throw redirect(302, '/ucp');
		}
		if (aha.status === 402) {
			return {
				error: await aha.text()
			};
		}
		if (aha.ok) {
			return {
				success: true
			};
		}
	} catch (err) {
		if (isRedirect(err)) {
			throw redirect(err.status, err.location);
		}
		return {
			error: true
		};
	}
}) satisfies LayoutServerLoad;
