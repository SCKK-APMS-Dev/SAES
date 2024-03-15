import express from 'express';
import { prisma } from './lib/prisma.js';

export const router = express.Router();

router.get('/:name/:type', async (req, res) => {
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7) - 7);
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	if (!req.params.type.split('_')[1]) {
		const kepek = await prisma.data.findMany({
			where: {
				owner: req.params.name.replace('_', ' '),
				type: req.params.type,
				status: 'elfogadva',
				date: {
					lte: nextPentek.toISOString(),
					gte: prevPentek.toISOString()
				}
			},
			select: {
				date: true,
				id: true
			}
		});
		res.send(kepek);
	} else {
		const kepek = await prisma.data.findMany({
			where: {
				owner: req.params.name.replace('_', ' '),
				type: req.params.type.split('_')[0],
				status: 'elfogadva',
				reason: req.params.type.split('_')[1],
				date: {
					lte: nextPentek.toISOString(),
					gte: prevPentek.toISOString()
				}
			},
			select: {
				date: true,
				id: true
			}
		});
		res.send(kepek);
	}
});
