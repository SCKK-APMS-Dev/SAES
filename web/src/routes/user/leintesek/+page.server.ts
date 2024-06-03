import { redirect, type Redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ parent, cookies }) => {
	const par = await parent();
	try {
		const aha = await fetch(
			par.layout.am ? `${apiUrl}/user/items/get` : `${apiUrl}/user/items/get?tipus=leintés`,
			{
				mode: 'no-cors',
				headers: {
					cookie: cookies.get('auth_token') as string
				}
			}
		);
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
