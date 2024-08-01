import { apiUrl } from '$lib/api';
import type { FullDBType } from '$lib/types';
import type { PageServerLoad } from './$types';

interface returnstat {
	stats: {
		potlekok: FullDBType[];
		leintesek: FullDBType[];
		szamlak: FullDBType[];
	};
	date: {
		next: Date;
		prev: Date;
	};
}

export const load = (async ({ parent, cookies }) => {
	await parent();
	const dcauth = cookies.get('auth_token');
	if (dcauth) {
		const mama = await fetch(`${apiUrl}/user/admin/stat?week=current`, {
			headers: {
				cookie: dcauth
			}
		});
		if (mama.ok) {
			const ret: returnstat = await mama.json();
			return {
				stats: ret.stats,
				date: {
					next: ret.date.next,
					prev: ret.date.prev
				}
			};
		}
	}
}) satisfies PageServerLoad;
