import { apiUrl, cdnUrl } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ params, url }) => {
	const aha = await fetch(
		`${apiUrl}/list?driver=${params.ppl.replace(
			'_',
			' '
		)}&tipus=${params.type}&faction=${url.searchParams.get('faction')}`
	);

	if (aha.ok) {
		const text = await aha.json();
		return {
			cucc: text,
			type: params.type,
			api: apiUrl,
			image: cdnUrl
		};
	}
}) satisfies PageServerLoad;
