import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = ({ cookies }) => {
	const muted = cookies.get('play_music');
	if (muted === 'false' || muted === 'undefined' || muted === undefined) {
		cookies.set('play_music', 'true', {
			path: '/',
			httpOnly: true,
			secure: true,
			maxAge: 2147483647
		});
		return new Response('true');
	} else if (muted === 'true') {
		cookies.set('play_music', 'false', {
			path: '/',
			httpOnly: true,
			secure: true,
			maxAge: 2147483647
		});
		return new Response('false');
	}
	return new Response('false');
};
