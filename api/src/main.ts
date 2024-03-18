import 'dotenv/config';
import express from 'express';
import cookieParser from 'cookie-parser';
import * as user from './user/main.js';
import * as image from './image.js';
import * as list from './list.js';
import { oauth } from './lib/discord.js';

const app = express();
const port = 3000;

app.use(express.json({ limit: '50mb' }));
app.use(express.text());
app.use(
	express.urlencoded({
		extended: true
	})
);
app.use('/user', user.router);
app.use('/img', image.router);
app.use('/list', list.router);

app.use(cookieParser(process.env.COOKIE_SECRET));

app.get('/', (req, res) => {
	res.send('SCKK API Szerver v0.0.1');
});

app.get('/cb', async (req, res) => {
	if (req.query.code) {
		try {
			const dcode = await oauth.tokenRequest({
				code: req.query.code as string,
				scope: 'identify',
				grantType: 'authorization_code'
			});
			res.cookie('sckk-dc-auth', dcode.access_token, {
				maxAge: dcode.expires_in * 1000,
				domain: process.env.NODE_DEV ? 'localhost' : 'sckk.hu'
			});
		} finally {
			res.redirect(process.env.NODE_DEV ? 'http://localhost:5173/user' : 'https://sckk.hu/user');
		}
	}
});

app.listen(port, async () => {
	const prevPentek = new Date();
	prevPentek.setDate(prevPentek.getDate() + ((5 - 7 - prevPentek.getDay()) % 7));
	const nextPentek = new Date(prevPentek.getTime() + 7 * 1000 * 60 * 60 * 24);
	prevPentek.setHours(22, 0, 0, 0);
	nextPentek.setHours(22, 0, 0, 0);
	console.log(prevPentek);
	console.log(nextPentek);
	console.log('http://localhost:3000');
});
