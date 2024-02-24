import { prisma } from '$lib/server/prisma';
import type { PageServerLoad } from './$types';

export const load = (async () => {
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
					db: 0
				};
			}
		}
	}
}) satisfies PageServerLoad;
