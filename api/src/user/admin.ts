import express from 'express';
import { adminAuth, basicAuth } from '../lib/discord.js';
import { prisma } from '../lib/db.js';

export const router = express.Router();

router.get('/', basicAuth, adminAuth, async (req, res) => {
	res.send(true);
});

router.get('/get/:type', basicAuth, adminAuth, async (req, res) => {
	const potlekok = await prisma.data.findMany({
		where: {
			type: req.params.type,
			status: req.headers.status ? (req.headers.status as string) : 'feltöltve'
		},
		select: {
			date: true,
			id: true,
			owner: true,
			status: true,
			reason: true,
			extra: true
		},
		orderBy: {
			date: 'desc'
		}
	});
	res.send(potlekok);
});

router.get('/get/current/:type', basicAuth, adminAuth, async (req, res) => {
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7));
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	const potlekok = await prisma.data.findMany({
		where: {
			type: req.params.type,
			status: req.headers.status ? (req.headers.status as string) : 'feltöltve',
			date: {
				lte: nextPentek.toISOString(),
				gte: prevPentek.toISOString()
			}
		},
		select: {
			date: true,
			id: true,
			owner: true,
			status: true,
			reason: true,
			extra: true
		},
		orderBy: {
			date: 'desc'
		}
	});
	res.send(potlekok);
});

router.get('/getall', basicAuth, adminAuth, async (req, res) => {
	const potlekok = await prisma.data.findMany({
		where: {
			status: req.headers.status ? (req.headers.status as string) : 'feltöltve'
		},
		select: {
			date: true,
			id: true,
			owner: true,
			status: true,
			reason: true,
			type: true
		},
		orderBy: {
			date: 'desc'
		}
	});
	res.send(potlekok);
});

router.post('/post', basicAuth, adminAuth, async (req, res) => {
	const body = await req.body;
	const upload = await prisma.data.update({
		where: {
			id: body.id
		},
		data: {
			status: body.status,
			reason: body.reason === '' ? null : body.reason
		},
		select: {
			date: true,
			id: true,
			owner: true,
			status: true,
			reason: true
		}
	});
	res.send(upload);
});
