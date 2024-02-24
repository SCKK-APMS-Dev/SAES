import { oauth } from '$lib/server/discord';
import { getTag } from '$lib/server/google';
import { prisma } from '$lib/server/prisma';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ cookies }) => {
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
				return new Response(JSON.stringify(potlekok));
			}
		}
	}
	return new Response('404');
};
