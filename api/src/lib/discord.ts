import DiscordOauth2 from 'discord-oauth2';
import 'dotenv/config';

export const oauth = new DiscordOauth2({
	clientId: process.env.DISCORD_ID,
	clientSecret: process.env.DISCORD_SECRET,
	redirectUri: process.env.NODE_DEV ? 'http://localhost:3000/cb' : 'https://api.sckk.hu/cb'
});

export async function getTag(
	discordid: string
): Promise<{ id: string; admin: string; name: string } | undefined> {
	const fatch = await fetch(`http://api.scms.hanrickio.com:5002/discord/player/${discordid}`);
	if (fatch.ok) {
		const ret = await fatch.json();
		if (!ret.error) {
			return {
				id: ret.Id,
				admin: ret.PermissionGroup,
				name: ret.PlayerName
			};
		}
	}
	return undefined;
}
