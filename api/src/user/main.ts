import express from 'express';
import { basicAuth, getTag, oauth } from '../lib/discord.js';
import * as admin from './admin.js';
import * as calls from './calls.js';
import { prisma } from '../lib/db.js';

export const router = express.Router();

router.use('/admin', admin.router);
router.use('/calls', calls.router);

router.get('/', basicAuth, async (req, res) => {
	res.send(req.doksi);
});

router.get('/auth', (req, res) => {
	res.redirect(
		oauth.generateAuthUrl({
			scope: 'identify'
		})
	);
});

router.get('/get', basicAuth, async (req, res) => {
	if (!req.headers.type) return res.sendStatus(404);
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7) - 7);
	const nextPentek = new Date(prevPentek.getTime() + 14 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	const cuccok = await prisma.data.findMany({
		where: {
			type: req.headers.type as string,
			owner: req.doksi.name as string,
			date: {
				lte: nextPentek.toISOString(),
				gte: prevPentek.toISOString()
			}
		},

		select: {
			date: true,
			id: true,
			status: true,
			reason: true,
			type: true,
			extra: true
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
});

router.post('/upload', basicAuth, async (req, res) => {
	if (!req.headers.type) return res.sendStatus(404);
	const body = await req.body;
	if (!body) return res.sendStatus(400);
	const kep = await prisma.data.create({
		data: {
			owner: req.doksi.name as string,
			kep: req.headers.type === 'leintés' ? JSON.stringify([body.img[0], body.img[1]]) : body.asd,
			type: req.headers.type as string,
			date: new Date(body.createdAt),
			extra: req.headers.extra ? (req.headers.extra as string) : null
		}
	});
	if (kep) {
		res.send(kep.id.toString());
	} else {
		res.sendStatus(400);
	}
});
