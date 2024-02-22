import type { PageServerLoad } from './$types';
import { prisma } from '$lib/server/prisma';

export const load = (async () => {
	// if (cookies.get('dc-auth')) {
	// 	return {};
	// }
	// throw redirect(302, 'https://api.ampix.hu/sckk/auth');
	const cucc = await prisma.users.findFirst({
		where: {
			id: 1
		}
	});
	return {
		cucc
	};
}) satisfies PageServerLoad;
