import type { PageServerLoad } from './$types';
import { prisma } from '$lib/server/prisma';

import { redirect } from '@sveltejs/kit';

import { oauth } from '$lib/server/discord';

export const load = (async ({ cookies }) => {
	// if (cookies.get('dc-auth')) {
	// 	return {};
	// }
	// throw redirect(302, 'https://api.ampix.hu/sckk/auth');
	const dcauth = cookies.get('dc-auth');
	if (dcauth) {
		const user = await oauth.getUser(dcauth);
		if (user) {
			const dev = await prisma.users.findUnique({
				where: {
					discordid: user.id
				}
			});
			if (dev) {
				return {
					dc: {
						name: dev.name
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
		throw redirect(
			302,
			oauth.generateAuthUrl({
				scope: 'identify'
			})
		);
	}
}) satisfies PageServerLoad;
