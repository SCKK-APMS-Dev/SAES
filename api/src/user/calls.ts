import express from 'express';
import { basicAuth } from '../lib/discord.js';
import { prisma } from '../lib/db.js';
import { getApiUrl } from '../lib/apis.js';

export const router = express.Router();

router.get('/', basicAuth, async (req, res) => {
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7));
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	try {
		const fatcs = await fetch(`${getApiUrl('erik')}/api/log/status/current`);
		if (fatcs.ok) {
			const eredmeny = await fatcs.json();
			const leintes = await prisma.data.findMany({
				where: {
					type: 'leintés',
					owner: req.doksi.name,
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
					owner: req.doksi.name,
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
					owner: req.doksi.name,
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
					owner: req.doksi.name,
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
				if (call.driver === req.doksi.name) {
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
});
router.get('/am', basicAuth, async (req, res) => {
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7));
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	try {
		const elfogadott_leintes = await prisma.data.findMany({
			where: {
				type: 'leintés',
				owner: req.doksi.name,
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
				owner: req.doksi.name,
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
				owner: req.doksi.name,
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
		if (!res.headersSent) {
			res.send({
				potlek: { de: dél.length, éj: éjsz.length }
			});
		}
	} catch (err) {
		res.sendStatus(400);
	}
});
