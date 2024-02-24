import { oauth } from '$lib/server/discord';
import { getTag } from '$lib/server/google';
import { prisma } from '$lib/server/prisma';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	const dcauth = cookies.get('dc-auth');
	if (dcauth) {
		const user = await oauth.getUser(dcauth);
		if (user) {
			const doksi = await getTag(user.id);
			if (doksi) {
				const potlekok = await prisma.data.findMany({
					where: {
						type: { in: ['délelőtti', 'éjszakai'] },
						owner: doksi.name as string
					}
				});
				return {
					potlek: URL.createObjectURL(new Blob([Buffer.from(potlekok[0].kep, 'utf8')]))
				};
			}
		}
	}
}) satisfies PageServerLoad;
