import { redirect, type Redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ parent, cookies }) => {
	const par = await parent();
	try {
		const aha = await fetch(par.layout.am ? `${apiUrl}/user/am/get` : `${apiUrl}/user/get`, {
			mode: 'no-cors',
			headers: {
				type: 'leintés',
				cookie: cookies.get('dc-auth') as string
			}
		});
		if (aha.status === 401) {
			throw redirect(302, 'noaccess');
		}

		if (aha.ok) {
			try {
				return {
					potlekok: await aha.json()
				};
			} catch {
				return {
					potlekok: undefined
				};
			}
		}
	} catch (err) {
		if ((err as Redirect).status) {
			throw redirect((err as Redirect).status, (err as Redirect).location);
		}
		return {
			error: 'Leintéseid lekérése sikertelen'
		};
	}
}) satisfies PageServerLoad;
