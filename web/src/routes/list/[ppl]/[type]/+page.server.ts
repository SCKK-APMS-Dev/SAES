import { apiUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ params }) => {
	const aha = await fetch(`${apiUrl}/list/${params.ppl}/${params.type}`);

	if (aha.ok) {
		return {
			cucc: await aha.json()
		};
	}
}) satisfies PageServerLoad;
