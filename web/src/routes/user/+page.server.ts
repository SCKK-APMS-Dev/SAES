// import { sheet } from '$lib/server/google';
// import { prisma } from '$lib/server/prisma';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ parent, cookies }) => {
	const par = await parent();
	try {
		const aha = await fetch(par.layout.am ? `${apiUrl}/user/calls/am` : `${apiUrl}/user/calls`, {
			mode: 'no-cors',
			headers: {
				cookie: cookies.get('dc-auth') as string
			}
		});
		if (aha.status === 401) {
			throw redirect(302, 'noaccess');
		}
		if (aha.status === 400) {
			return {
				error: 'Központi API szerver elérése sikertelen'
			};
		}
		if (aha.ok) {
			const text = await aha.json();
			return {
				calls: text
			};
		}
	} catch {
		return {
			error: 'Weboldal API szervere elérése sikertelen'
		};
	}
}) satisfies PageServerLoad;
