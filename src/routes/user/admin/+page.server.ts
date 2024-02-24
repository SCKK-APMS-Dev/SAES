import type { PageServerLoad } from './$types';

import 'dotenv/config';
import { redirect } from '@sveltejs/kit';

import { sheet } from '$lib/server/google';

export const load = (async ({ parent }) => {
	const par = await parent();
	if (par.layout?.doksi) {
		await sheet.loadCells();
		if (par.layout.doksi.rang === 'admin') {
			const users: { name: string; discordid: string; rang: string }[] = [];
			for (let i = 4; i < sheet.columnCount; i++) {
				if (sheet.getCellByA1(`A${i}`).value === 'Összesen') {
					break;
				}
				if (sheet.getCellByA1(`A${i}`).value && sheet.getCellByA1(`A${i}`).note) {
					const sanyi = {
						name: sheet.getCellByA1(`A${i}`).value as string,
						discordid: sheet.getCellByA1(`A${i}`).note.split('\n')[0] as string,
						rang: sheet.getCellByA1(`A${i}`).note.split('\n')[1]
							? sheet.getCellByA1(`A${i}`).note.split('\n')[1]
							: 'tag'
					};
					users.push(sanyi);
				}
			}
			return {
				admin: {
					users
				}
			};
		}
		throw redirect(302, '/user');
	}
	throw redirect(302, '/auth');
}) satisfies PageServerLoad;
