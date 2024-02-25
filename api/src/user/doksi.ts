import express from 'express';
import { oauth } from '../lib/discord.ts';
import { getTag, sheet } from '../lib/google.ts';
import { prisma } from '../lib/prisma.ts';

export const router = express.Router();

router.get('/', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	let cookie: string | undefined = undefined;
	for (const kuki of JSON.parse(req.headers.cookie)) {
		if (kuki.name === 'sckk-dc-auth') {
			cookie = kuki.value;
		}
	}
	if (!cookie) return res.sendStatus(404);
	const user = await oauth.getUser(cookie);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			await sheet.loadCells(`B${doksi?.row}:F${doksi?.row}`);
			const app_calls = sheet.getCellByA1(`B${doksi.row}`);
			const leintesek = sheet.getCellByA1(`C${doksi.row}`);
			const all_calls = sheet.getCellByA1(`D${doksi.row}`);
			const délelőtti = sheet.getCellByA1(`E${doksi.row}`);
			const éjszakai = sheet.getCellByA1(`F${doksi.row}`);
			const elfogadott = await prisma.data.findMany({
				where: {
					OR: [
						{ type: 'délelőtti', owner: doksi.name as string, status: 'elfogadva' },
						{ type: 'éjszakai', owner: doksi.name as string, status: 'elfogadva' }
					]
				},
				select: {
					type: true
				}
			});
			res.send({
				page: {
					calls: {
						app: app_calls.value === null ? 0 : app_calls.value,
						leint: leintesek.value === null ? 0 : leintesek.value,
						all: all_calls.value === null ? 0 : all_calls.value
					},
					potlek: {
						délelőtti: {
							all: délelőtti.value === null ? 0 : délelőtti.value,
							elfogadott:
								elfogadott.filter((item) => item.type === 'délelőtti').length > 0
									? elfogadott.filter((item) => item.type === 'délelőtti').length
									: 0
						},
						éjszakai: {
							all: éjszakai.value === null ? 0 : éjszakai.value,
							elfogadott:
								elfogadott.filter((item) => item.type === 'éjszakai').length > 0
									? elfogadott.filter((item) => item.type === 'éjszakai').length
									: 0
						}
					}
				}
			});
		} else {
			res.sendStatus(401);
		}
	} else {
		res.sendStatus(404);
	}
});
