import { apiUrl } from '$lib/api';
import { redirect, type Redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = (async ({ parent, cookies }) => {
	await parent();
	try {
		const aha = await fetch(`${apiUrl}/user/admin/potlekok`, {
			mode: 'no-cors',
			headers: {
				cookie: cookies.get('sckk-dc-auth') as string
			}
		});
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
