import type { PageServerLoad } from './$types';

import { oauth } from '$lib/server/discord';
import { redirect } from '@sveltejs/kit';

export const load = (async ({ cookies, url }) => {
	const code = url.searchParams.get('code');
	if (code) {
		const dcode = await oauth.tokenRequest({
			code,
			scope: 'identify',
			grantType: 'authorization_code'
		});
		cookies.set('dc-auth', dcode.access_token, { maxAge: dcode.expires_in, path: '/' });
		throw redirect(302, '/');
	}
}) satisfies PageServerLoad;
