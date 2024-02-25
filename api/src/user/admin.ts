import express from 'express';
import { getTag, loadRowes, sheet } from '../lib/google.ts';
import { oauth } from '../lib/discord.ts';

export const router = express.Router();

router.get('/', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	if (user) {
		const doksi = await getTag(user.id);
		const rowes = (await loadRowes()) as number;
		if (doksi) {
			if (doksi.rang === 'admin') {
				const users: { name: string; discordid: string; rang: string }[] = [];
				sheet.loadCells(`A4:A${rowes}`);
				for (let i = 4; i < rowes; i++) {
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
				res.send(JSON.stringify(users));
			}
		} else {
			res.sendStatus(401);
		}
	} else {
		res.sendStatus(404);
	}
});
