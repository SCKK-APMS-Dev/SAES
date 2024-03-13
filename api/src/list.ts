import express from 'express';
import { prisma } from './lib/prisma.js';

export const router = express.Router();

router.get('/:name/:type', async (req, res) => {
	if (!req.params.type.split('_')[1]) {
		const kepek = await prisma.data.findMany({
			where: {
				owner: req.params.name.replace('_', ' '),
				type: req.params.type,
				status: 'elfogadva'
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
				reason: req.params.type.split('_')[1]
			},
			select: {
				date: true,
				id: true
			}
		});
		res.send(kepek);
	}
});
