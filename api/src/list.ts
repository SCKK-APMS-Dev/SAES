import express from 'express';
import { prisma } from './lib/prisma.ts';

export const router = express.Router();

router.get('/:name/:type', async (req, res) => {
	const kepek = await prisma.data.findMany({
		where: {
			owner: req.params.name.replace('_', ' '),
			type: req.params.type
		},
		select: {
			date: true,
			id: true
		}
	});
	res.send(kepek);
});
