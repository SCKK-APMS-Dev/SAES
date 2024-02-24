import express from 'express';
import { oauth } from '../lib/discord.ts';
import { getTag } from '../lib/google.ts';

export const router = express.Router();

router.get('/', async (req, res) => {
	if (!req.headers.cookie) return res.sendStatus(404);
	let cookie: string | undefined = undefined;
	for (const kuki of JSON.parse(req.headers.cookie)) {
		if (kuki.name === 'sckk-dc-auth') {
			cookie = kuki.value;
		}
	}
	if (!cookie) return res.sendStatus(404);
	const user = await oauth.getUser(cookie);
	if (user) {
		const doksi = await getTag(user.id);
		if (doksi) {
			res.send(doksi);
		} else {
			res.sendStatus(401);
		}
	} else {
		res.sendStatus(404);
	}
});

router.get('/auth', (req, res) => {
	res.redirect(
		oauth.generateAuthUrl({
			scope: 'identify'
		})
	);
});
