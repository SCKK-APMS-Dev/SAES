import { oauth } from '$lib/server/discord';
import { getTag } from '$lib/server/google';
import type { Actions, PageServerLoad } from './$types';

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

export const actions: Actions = {
	upload: async ({ request }) => {
		const formdata = await request.formData();

		const fsa = formdata.get('file') as File;
		const reader = new FileReader();

		reader.onloadend = () => {
			console.log(reader.result);
		};
		reader.readAsDataURL(fsa);
	}
};
