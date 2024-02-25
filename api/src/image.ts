import express from 'express';
import { prisma } from './lib/prisma.ts';

export const router = express.Router();

router.get('/data/:name', async (req, res) => {
	const param = req.params.name.split('.')[0];
	const file = await prisma.data.findUnique({
		where: {
			id: Number(param)
		}
	});
	if (file) {
		// Set content type in response headers
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
});
