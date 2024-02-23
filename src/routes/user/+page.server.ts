import { oauth } from '$lib/server/discord';
import { getTag, sheet } from '$lib/server/google';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	const dcauth = cookies.get('dc-auth') as string;
	const user = await oauth.getUser(dcauth);
	if (user) {
		await sheet.loadCells();
		const doksi = await getTag(user.id);
		if (doksi) {
			const app_calls = sheet.getCell(doksi?.row, 1);
			const leintesek = sheet.getCell(doksi?.row, 2);
			const all_calls = sheet.getCell(doksi?.row, 3);
			const délelőtti = sheet.getCell(doksi?.row, 4);
			const éjszakai = sheet.getCell(doksi?.row, 5);
			return {
				page: {
					calls: {
						app: app_calls.value === null ? 0 : app_calls.value,
						leint: leintesek.value === null ? 0 : leintesek.value,
						all: all_calls.value === null ? 0 : all_calls.value
					},
					potlek: {
						délelőtti: délelőtti.value === null ? 'nincs' : délelőtti.value,
						éjszakai: éjszakai.value === null ? 'nincs' : éjszakai.value
					}
				}
			};
		}
	}
}) satisfies PageServerLoad;
