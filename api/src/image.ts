import express from 'express';
import { prisma } from './lib/db.js';
import path from 'node:path';

export const router = express.Router();

router.get('/data/:name', async (req, res) => {
	const param = req.params.name.split('.')[0];
	const file = await prisma.data.findUnique({
		where: {
			id: Number(param) ? Number(param) : 0
		}
	});
	if (file) {
		if (file.type !== 'leintés') {
			res.sendFile(process.env.NODE_DEV ? path.resolve(`data/${file.kep}`) : `/data/${file.kep}`);
		} else {
			res.sendStatus(404);
		}
	} else {
		res.sendStatus(404);
	}
});

router.get('/data/:index/:jona', async (req, res) => {
	const param = req.params.index.split('.')[0];
	const count = Number(req.params.jona) ? Number(req.params.jona) : 0;
	const file = await prisma.data.findUnique({
		where: {
			id: Number(param) ? Number(param) : 0
		}
	});
	if (file) {
		if (file.type === 'leintés' && 2 > count) {
			res.sendFile(
				process.env.NODE_DEV
					? path.resolve(`data/${JSON.parse(file.kep)[count]}`)
					: `/data/${JSON.parse(file.kep)[count]}`
			);
		} else {
			res.sendStatus(404);
		}
	} else {
		res.sendStatus(404);
	}
});
