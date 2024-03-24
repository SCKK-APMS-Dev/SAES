import express from 'express';
import { basicAuth, getTag, oauth } from '../lib/discord.js';
import * as admin from './admin.js';
import * as calls from './calls.js';
import { prisma } from '../lib/db.js';
import multer from 'multer';

const storage = multer.diskStorage({
	destination: (req, file, cb) => {
		cb(null, process.env.NODE_DEV ? 'data' : '/data');
	},
	filename: (req, file, cb) => {
		const uniqueSuffix = `${Date.now()}-${Math.round(Math.random() * 1e9)}`;
		cb(null, `${file.fieldname}-${uniqueSuffix}.${file.originalname}`);
	}
});

const upload = multer({ storage });
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

import { createWorker } from 'tesseract.js';

router.post('/upload', basicAuth, upload.array('files'), async (req, res) => {
	if (!req.headers.type) return res.sendStatus(404);
	if (!req.headers.dates) return res.sendStatus(404);
	const files: string[] = [];
	if (req.headers.type !== 'leintés') {
		(req.files as Express.Multer.File[]).forEach(async (val, i) => {
			if (req.headers.type === 'számla') {
				const worker = await createWorker('hun');
				const ret = await worker.recognize(
					process.env.NODE_DEV ? `data/${val.filename}` : `/data/${val.filename}`
				);
				const moni = ret.data.text
					.split('\n')
					.findLastIndex((el) => el.includes('Végösszeg:') && el.endsWith('$'));
				await worker.terminate();
				const kep = await prisma.data.create({
					data: {
						owner: req.doksi.name as string,
						kep: val.filename,
						type: req.headers.type as string,
						date: new Date(Number(JSON.parse(req.headers.dates as string)[i])).toISOString(),
						extra:
							moni === -1
								? null
								: ret.data.text
										.split('\n')
										[moni].split(':')[1]
										.trim()
										.replace(' ', '')
										.slice(undefined, -2)
					}
				});
				if (kep) {
					files.push(kep.id.toString());
					if (i === (req.files?.length as number) - 1) {
						res.send(JSON.stringify(files));
					}
				}
			} else {
				const kep = await prisma.data.create({
					data: {
						owner: req.doksi.name as string,
						kep: val.filename,
						type: req.headers.type as string,
						date: new Date(Number(JSON.parse(req.headers.dates as string)[i])).toISOString(),
						extra: req.headers.extra ? (req.headers.extra as string) : null
					}
				});
				if (kep) {
					files.push(kep.id.toString());
					if (i === (req.files?.length as number) - 1) {
						res.send(JSON.stringify(files));
					}
				}
			}
		});
	} else {
		for (let i = 0; i < (req.files?.length as number) / 2; i++) {
			if (Array.isArray(req.files)) {
				const kep = await prisma.data.create({
					data: {
						owner: req.doksi.name as string,
						kep: JSON.stringify([req.files[i * 2].filename, req.files[i * 2 + 1].filename]),
						type: req.headers.type as string,
						date: new Date(Number(JSON.parse(req.headers.dates as string)[i * 2])).toISOString(),
						extra: req.headers.extra ? (req.headers.extra as string) : null
					}
				});
				if (kep) {
					files.push(kep.id.toString());
					if (i === (req.files?.length as number) / 2 - 1) {
						res.send(JSON.stringify(files));
					}
				}
			}
		}
	}
});
