import DiscordOauth2 from 'discord-oauth2';
import 'dotenv/config';
import type { RequestHandler } from 'express';

export const oauth = new DiscordOauth2({
	clientId: process.env.DISCORD_ID,
	clientSecret: process.env.DISCORD_SECRET,
	redirectUri: process.env.NODE_DEV ? 'http://localhost:3000/cb' : 'https://api.sckk.hu/cb'
});

export async function getTag(
	discordid: string
): Promise<{ id: number; admin: boolean; name: string } | undefined> {
	try {
		const fatch = await fetch(`http://192.168.100.148:5002/discord/player/${discordid}`);
		if (fatch.ok) {
			const ret = await fatch.json();
			if (!ret.error) {
				return {
					id: ret.Id,
					admin: ret.PermissionGroup === 1 ? true : false,
					name: ret.PlayerName
				};
			}
		}
	} catch {
		return undefined;
	}
}

declare global {
	namespace Express {
		// Inject additional properties on express.Request
		interface Request {
			/**
			 * This request's secret.
			 * Optionally set by cookie-parser if secret(s) are provided.  Can be used by other middleware.
			 * [Declaration merging](https://www.typescriptlang.org/docs/handbook/declaration-merging.html) can be used to add your own properties.
			 */
			admin: boolean;
			doksi: {
				id: number;
				admin: boolean;
				name: string;
			};
		}
	}
}

export const basicAuth: RequestHandler = async (req, res, next) => {
	if (req.headers.cookie === 'undefined' || !req.headers.cookie) return res.sendStatus(404);
	const user = await oauth.getUser(req.headers.cookie);
	if (!user) return res.sendStatus(404);
	const doksi = await getTag(user.id);
	if (!doksi) return res.sendStatus(401);
	req.doksi = doksi;
	req.admin = false;
	return next();
};

export const adminAuth: RequestHandler = async (req, res, next) => {
	if (req.doksi.admin) {
		req.admin = true;
		return next();
	}
	res.sendStatus(401);
};
