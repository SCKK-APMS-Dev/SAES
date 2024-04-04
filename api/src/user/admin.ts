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
			status: req.headers.status ? (req.headers.status as string) : 'feltöltve',
			am: false
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

router.get('/am/get/:type', basicAuth, adminAuth, async (req, res) => {
	const potlekok = await prisma.data.findMany({
		where: {
			type: req.params.type,
			status: req.headers.status ? (req.headers.status as string) : 'feltöltve',
			am: true
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
			extra: true,
			am: true
		},
		orderBy: {
			date: 'desc'
		}
	});
	res.send(potlekok);
});

router.get('/am/get/current/:type', basicAuth, adminAuth, async (req, res) => {
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7));
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	const potlekok = await prisma.data.findMany({
		where: {
			type: req.params.type,
			status: req.headers.status ? (req.headers.status as string) : 'feltöltve',
			am: true,
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
			am: true,
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
			status: req.headers.status ? (req.headers.status as string) : 'feltöltve',
			am: false
		},
		select: {
			date: true,
			id: true,
			owner: true,
			status: true,
			reason: true,
			type: true,
			extra: true
		},
		orderBy: {
			date: 'desc'
		}
	});
	res.send(potlekok);
});

router.get('/am/getall', basicAuth, adminAuth, async (req, res) => {
	const potlekok = await prisma.data.findMany({
		where: {
			status: req.headers.status ? (req.headers.status as string) : 'feltöltve',
			am: true
		},
		select: {
			date: true,
			id: true,
			owner: true,
			status: true,
			reason: true,
			type: true,
			extra: true
		},
		orderBy: {
			date: 'desc'
		}
	});
	console.log(potlekok);
	res.send(potlekok);
});

router.post('/post', basicAuth, adminAuth, async (req, res) => {
	const body = await req.body;
	const upload = await prisma.data.update({
		where: {
			id: body.id,
			am: body.am
		},
		data: {
			status: body.status,
			reason: body.reason === '' ? null : body.reason,
			extra: body.extra === '' ? null : body.extra
		},
		select: {
			date: true,
			id: true,
			owner: true,
			status: true,
			reason: true,
			extra: true
		}
	});
	res.send(upload);
});
