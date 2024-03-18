import { apiUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ parent, cookies }) => {
	await parent();
	const dcauth = cookies.get('sckk-dc-auth');
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7) - 7);
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	if (dcauth) {
		const mama = await fetch(`${apiUrl}/user/admin/getall`, {
			headers: {
				cookie: dcauth,
				status: 'elfogadva'
			}
		});
		if (mama.ok) {
			return {
				stats: await mama.json(),
				date: {
					next: nextPentek,
					prev: prevPentek
				}
			};
		}
	}
}) satisfies PageServerLoad;
