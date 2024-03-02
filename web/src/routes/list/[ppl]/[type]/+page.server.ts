import { apiUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ params }) => {
	const aha = await fetch(`${apiUrl}/list/${params.ppl}/${params.type}`);
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7) - 7);
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);

	if (aha.ok) {
		return {
			cucc: await aha.json(),
			prev: prevPentek,
			next: nextPentek
		};
	}
}) satisfies PageServerLoad;
