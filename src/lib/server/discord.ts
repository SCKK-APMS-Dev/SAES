import DiscordOauth2 from 'discord-oauth2';

export const oauth = new DiscordOauth2({
	clientId: '1210268158792048690',
	clientSecret: '86bpca8walfcKIj1J9uF3XqnWwSndlTS',
	redirectUri:
		process.env.NODE_ENV === 'development' ? 'http://localhost:5173/cb' : 'https://sckk.ampix.hu/cb'
});
