import express from 'express';
import { basicAuth, getTag, oauth } from '../lib/discord.js';
import * as admin from './admin.js';
import * as calls from './calls.js';

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
