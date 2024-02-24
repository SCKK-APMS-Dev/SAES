import { oauth } from '$lib/server/discord';
import { getTag } from '$lib/server/google';
import { prisma } from '$lib/server/prisma';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.text();
	const dcauth = cookies.get('dc-auth');
	if (dcauth) {
		const user = await oauth.getUser(dcauth);
		if (user) {
			const doksi = await getTag(user.id);
			if (doksi) {
				await prisma.data.create({
					data: {
						owner: doksi.name as string,
						type: 'délelőtti',
						kep: body
					}
				});
			}
		}
	}
	return new Response(body);
};
