import DiscordOauth2 from 'discord-oauth2';
import 'dotenv/config';

export const oauth = new DiscordOauth2({
	clientId: process.env.DISCORD_ID,
	clientSecret: process.env.DISCORD_SECRET,
	redirectUri:
		process.env.NODE_ENV === 'development' ? 'http://localhost:5173/cb' : 'https://sckk.ampix.hu/cb'
});
