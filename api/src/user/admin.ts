import express from 'express';
import { getTag, oauth } from '../lib/discord.js';
import { prisma } from '../lib/prisma.js';

export const router = express.Router();

router.get('/', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	try {
		const user = await oauth.getUser(req.headers.cookie);
		if (user) {
			const doksi = await getTag(user.id);
			if (doksi) {
				if (doksi.admin) {
					res.send(true);
				} else {
					res.sendStatus(401);
				}
			} else {
				res.sendStatus(401);
			}
		} else {
			res.sendStatus(404);
		}
	} catch {
		res.sendStatus(400);
	}
});

router.get('/get/:type', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			if (doksi.admin) {
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
						reason: true
					},
					orderBy: {
						date: 'desc'
					}
				});
				res.send(potlekok);
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

router.get('/getall', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			if (doksi.admin) {
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

router.post('/post', async (req, res) => {
	const body = await req.body;
	if (!req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			if (doksi.admin) {
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
			}
		}
	}
});
