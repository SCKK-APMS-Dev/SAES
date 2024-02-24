import { oauth } from '$lib/server/discord';
import { getTag } from '$lib/server/google';
import type { LayoutServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';

export const load = (async ({ cookies }) => {
	const dcauth = cookies.get('dc-auth');
	if (dcauth) {
		const user = await oauth.getUser(dcauth);
		if (user) {
			const doksi = await getTag(user.id);
			if (doksi) {
				return {
					layout: {
						doksi
					}
				};
			}
			throw redirect(302, 'noaccess');
		}
	} else {
		throw redirect(
			302,
			oauth.generateAuthUrl({
				scope: 'identify'
			})
		);
	}
}) satisfies LayoutServerLoad;
