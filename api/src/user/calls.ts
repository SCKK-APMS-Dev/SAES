import express from 'express';
import { getTag, oauth } from '../lib/discord.js';
import { prisma } from '../lib/prisma.js';

export const router = express.Router();

router.get('/', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7));
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			try {
				const fatcs = await fetch('https://thfsystem.com/api/log/status/current');
				if (fatcs.ok) {
					const eredmeny = await fatcs.json();
					const leintes = await prisma.data.findMany({
						where: {
							type: 'leintés',
							owner: doksi.name,
							status: {
								not: 'elutasítva'
							},
							date: {
								lte: nextPentek.toISOString(),
								gte: prevPentek.toISOString()
							}
						},
						select: {
							id: true
						}
					});
					const elfogadott_leintes = await prisma.data.findMany({
						where: {
							type: 'leintés',
							owner: doksi.name,
							status: 'elfogadva',
							date: {
								lte: nextPentek.toISOString(),
								gte: prevPentek.toISOString()
							}
						},
						select: {
							id: true
						}
					});
					const dél = await prisma.data.findMany({
						where: {
							type: 'pótlék',
							reason: 'délelőtti',
							owner: doksi.name,
							status: 'elfogadva',
							date: {
								lte: nextPentek.toISOString(),
								gte: prevPentek.toISOString()
							}
						},
						select: {
							id: true
						}
					});
					const éjsz = await prisma.data.findMany({
						where: {
							type: 'pótlék',
							reason: 'éjszakai',
							owner: doksi.name,
							status: 'elfogadva',
							date: {
								lte: nextPentek.toISOString(),
								gte: prevPentek.toISOString()
							}
						},
						select: {
							id: true
						}
					});
					for (const call of eredmeny) {
						if (call.driver === doksi.name) {
							res.send({
								app: call.count.toString(),
								leintes: {
									all: leintes.length,
									elfogadott: elfogadott_leintes.length
								},
								potlek: {
									de: dél.length,
									éj: éjsz.length
								}
							});
							break;
						}
					}
					if (!res.headersSent) {
						res.send({
							app: 0,
							leintes: { áll: leintes.length, elfogadott: elfogadott_leintes.length },
							potlek: { de: dél.length, éj: éjsz.length }
						});
					}
				} else {
					res.sendStatus(400);
				}
			} catch (err) {
				res.sendStatus(400);
			}
		} else {
			res.sendStatus(401);
		}
	} else {
		res.sendStatus(404);
	}
});
