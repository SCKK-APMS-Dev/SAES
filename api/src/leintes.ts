import express from 'express';
import { basicAuth, getTag, oauth } from './lib/discord.js';
import { prisma } from './lib/db.js';

export const router = express.Router();

router.get('/', basicAuth, async (req, res) => {
	const cuccok = await prisma.data.findMany({
		where: {
			owner: req.doksi.name as string,
			type: 'leintés'
		},
		select: {
			date: true,
			id: true,
			status: true,
			reason: true
		},
		orderBy: {
			date: 'desc'
		}
	});
	if (cuccok[0]) {
		res.send(cuccok);
	} else {
		res.sendStatus(204);
	}
	res.sendStatus(404);
});

router.post('/upload', basicAuth, async (req, res) => {
	const body = await req.body;
	if (!body) return res.sendStatus(400);
	const kep = await prisma.data.create({
		data: {
			owner: req.doksi.name as string,
			kep: JSON.stringify([body.img[0], body.img[1]]),
			date: new Date(body.createdAt),
			type: 'leintés'
		}
	});
	if (kep) {
		res.send(kep.id.toString());
	} else {
		res.sendStatus(400);
	}
});
