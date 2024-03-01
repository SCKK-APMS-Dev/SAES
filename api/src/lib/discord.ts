import DiscordOauth2 from 'discord-oauth2';
import 'dotenv/config';

export const oauth = new DiscordOauth2({
	clientId: process.env.DISCORD_ID,
	clientSecret: process.env.DISCORD_SECRET,
	redirectUri: process.env.NODE_DEV ? 'http://localhost:3000/cb' : 'https://api.sckk.hu/cb'
});
