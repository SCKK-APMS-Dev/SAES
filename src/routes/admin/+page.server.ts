import type { PageServerLoad } from './$types';

import 'dotenv/config';
import { redirect } from '@sveltejs/kit';

import { oauth } from '$lib/server/discord';

import { getTag, sheet } from '$lib/server/google';

export const load = (async ({ cookies }) => {
	const dcauth = cookies.get('dc-auth');
	if (dcauth) {
		const user = await oauth.getUser(dcauth);
		if (user) {
			await sheet.loadCells();
			const tag = await getTag(user.id);

			if (tag?.rang === 'admin') {
				const users: { name: string; discordid: string; rang: string }[] = [];
				for (let i = 4; i < sheet.columnCount; i++) {
					if (sheet.getCellByA1(`A${i}`).value === 'Összesen') {
						break;
					}
					if (sheet.getCellByA1(`A${i}`).value) {
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
			return {
				error: {
					msg: 'Nincs hozzáférésed!'
				}
			};
		}
	} else {
		throw redirect(302, '/auth');
	}
}) satisfies PageServerLoad;
