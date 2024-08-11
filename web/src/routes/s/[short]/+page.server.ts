import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api';

export const load = (async ({ params }) => {
	const shorts = await fetch(`${apiUrl}/shorts?key=${process.env.secret_key}`);
	const res: { short: string; url: string }[] = await shorts.json();
	if (res.some((val) => val.short === params.short)) {
		throw redirect(307, res[res.findIndex((val) => val.short === params.short)].url);
	} else {
		throw error(404, 'Ez a rövidítés nem található!');
	}
}) satisfies PageServerLoad;
