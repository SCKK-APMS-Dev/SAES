import 'dotenv/config';
import express from 'express';
import cookieParser from 'cookie-parser';
import * as user from './user/main.ts';
import * as image from './image.ts';
import { oauth } from './lib/discord.ts';

const app = express();
const port = 3000;

app.use(express.json());
app.use(
	express.urlencoded({
		extended: true
	})
);
app.use('/user', user.router);
app.use('/img', image.router);

app.use(cookieParser(process.env.COOKIE_SECRET));

app.get('/', (req, res) => {
	res.send('SCKK API Szerver');
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
				domain: 'localhost'
			});
		} finally {
			res.redirect(
				process.env.NODE_DEV ? 'http://localhost:5173/user' : 'https://sckk.ampix.hu/user'
			);
		}
	}
});

app.listen(port, () => {
	console.log('http://localhost:3000');
});
