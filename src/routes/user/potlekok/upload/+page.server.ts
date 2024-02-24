import { oauth } from '$lib/server/discord';
import { getTag } from '$lib/server/google';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	const dcauth = cookies.get('dc-auth');
	if (dcauth) {
		const user = await oauth.getUser(dcauth);
		if (user) {
			const doksi = await getTag(user.id);
			if (doksi) {
				return {
					db: 0
				};
			}
		}
	}
}) satisfies PageServerLoad;
