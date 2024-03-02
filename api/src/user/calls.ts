import express from 'express';
import { getTag, oauth } from '../lib/discord.ts';
import { prisma } from '../lib/prisma.ts';

export const router = express.Router();

router.get('/', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			const fatcs = await fetch('https://thfsystem.com/api/log/status/current');
			if (fatcs.ok) {
				const eredmeny = await fatcs.json();
				for (const call of eredmeny) {
					if (call.driver === doksi.name) {
						res.send(call.count.toString());
						break;
					}
				}
				if (!res.headersSent) {
					res.send('nincs');
				}
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
