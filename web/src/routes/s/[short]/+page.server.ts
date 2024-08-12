import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ params }) => {
	const shorts = await fetch(`${apiUrl}/shorts`, {
		headers: {
			'secret-key': process.env.secret_key as string
		}
	});
	if (shorts.ok) {
		const res: { short: string; url: string }[] = await shorts.json();
		if (res.some((val) => val.short === params.short)) {
			throw redirect(307, res[res.findIndex((val) => val.short === params.short)].url);
		}
		throw error(404, 'Ez a rövidítés nem található!');
	}
	throw error(400, await shorts.text());
}) satisfies PageServerLoad;
