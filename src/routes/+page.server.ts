import type { PageServerLoad } from './$types';

import { oauth } from '$lib/server/discord';

import { getTag } from '$lib/server/google';

export const load = (async ({ cookies }) => {
	// if (cookies.get('dc-auth')) {
	// 	return {};
	// }
	// throw redirect(302, 'https://api.ampix.hu/sckk/auth');
	const dcauth = cookies.get('dc-auth');
	if (dcauth) {
		const user = await oauth.getUser(dcauth);
		if (user) {
			const dev = await getTag(user.id);
			if (dev) {
				return {
					dc: {
						name: dev.name,
						role: dev.rang
					}
				};
			}
			return {
				error: {
					msg: 'Nincs hozzáférésed!'
				}
			};
		}
	} else {
		return {
			noauth: true
		};
	}
}) satisfies PageServerLoad;
