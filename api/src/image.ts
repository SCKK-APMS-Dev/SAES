import express from 'express';
import { prisma } from './lib/prisma.ts';

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
			const base64Data = file.kep.replace(/^data:image\/\w+;base64,/, '');

			// Decode base64 data
			const decodedImage = Buffer.from(base64Data, 'base64');

			// Set content type in response headers
			res.setHeader('Content-Type', 'image/png');

			// Send the image data in the response
			res.send(decodedImage);
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
		if (file.type === 'leintés' && 3 > count) {
			const files = JSON.parse(file.kep);
			res.setHeader('Content-Type', 'image/png');
			const base64Data = files[count - 1].replace(/^data:image\/\w+;base64,/, '');

			// Decode base64 data
			const decodedImage = Buffer.from(base64Data, 'base64');

			// Set content type in response headers

			// Send the image data in the response
			res.send(decodedImage);
		} else {
			res.sendStatus(404);
		}
	} else {
		res.sendStatus(404);
	}
});
