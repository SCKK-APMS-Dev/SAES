import express from 'express';
import { getTag, loadRowes, sheet } from '../lib/google.ts';
import { oauth } from '../lib/discord.ts';
import { prisma } from '../lib/prisma.ts';

export const router = express.Router();

router.get('/', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	try {
		const user = await oauth.getUser(req.headers.cookie);
		if (user) {
			const doksi = await getTag(user.id);
			const rowes = (await loadRowes()) as number;
			if (doksi) {
				if (doksi.rang === 'admin') {
					const users: { name: string; discordid: string; rang: string }[] = [];
					await sheet.loadCells(`A4:A${rowes}`);
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
				} else {
					res.sendStatus(401);
				}
			} else {
				res.sendStatus(401);
			}
		} else {
			res.sendStatus(404);
		}
	} catch {
		res.sendStatus(400);
	}
});

router.get('/get/:type', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			if (doksi.rang === 'admin') {
				const potlekok = await prisma.data.findMany({
					where: {
						type: req.params.type,
						status: req.headers.status ? (req.headers.status as string) : 'feltöltve'
					},
					select: {
						date: true,
						id: true,
						owner: true,
						status: true,
						reason: true
					},
					orderBy: {
						date: 'desc'
					}
				});
				res.send(potlekok);
			} else {
				res.sendStatus(401);
			}
		} else {
			res.sendStatus(401);
		}
	} else {
		res.sendStatus(404);
	}
});

router.post('/post', async (req, res) => {
	const body = await req.body;
	if (!req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			if (doksi.rang === 'admin') {
				const upload = await prisma.data.update({
					where: {
						id: body.id
					},
					data: {
						status: body.status,
						reason: body.reason === '' ? null : body.reason
					},
					select: {
						date: true,
						id: true,
						owner: true,
						status: true,
						reason: true
					}
				});
				res.send(upload);
			}
		}
	}
});
