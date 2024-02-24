import { prisma } from '$lib/server/prisma';
import type { PageServerLoad } from './$types';

export const load = (async ({ parent }) => {
	const par = await parent();
	if (par.layout?.doksi) {
		const potlekok = await prisma.data.findMany({
			where: {
				type: { in: ['délelőtti', 'éjszakai'] },
				owner: par.layout.doksi.name as string
			}
		});
		return {
			potlekok
		};
	}
}) satisfies PageServerLoad;
