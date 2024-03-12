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
				const leintes = await prisma.data.findMany({
					where: {
						type: 'leintés',
						owner: doksi.name,
						status: {
							not: 'elutasítva'
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
						status: 'elfogadva'
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
						status: 'elfogadva'
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
						status: 'elfogadva'
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
					res.send({app: 0,leintes: {áll: leintes.length,elfogadott:elfogadott_leintes.length},potlek: {de: dél.length,éj: éjsz.length}});
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
