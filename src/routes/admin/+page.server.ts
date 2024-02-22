import type { PageServerLoad } from './$types';

import { prisma } from '$lib/server/prisma';

import { redirect } from '@sveltejs/kit';

import { oauth } from '$lib/server/discord';

export const load = (async ({ cookies }) => {
	const dcauth = cookies.get('dc-auth');
	if (dcauth) {
		const user = await oauth.getUser(dcauth);
		if (user) {
			const dev = await prisma.users.findUnique({
				where: {
					discordid: user.id
				}
			});
			if (dev?.role === 'admin') {
				const tagok = await prisma.users.findMany();
				return {
					admin: {
						tagok
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
		throw redirect(302, '/');
	}
}) satisfies PageServerLoad;
