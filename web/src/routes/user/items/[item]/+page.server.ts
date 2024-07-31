import { redirect, type Redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { apiUrl } from '$lib/api';
import { Reeler_keys, Reeler_vals } from '$lib/public';

export const load = (async ({ parent, cookies, params, url }) => {
	await parent();
	try {
		if (Reeler_keys.includes(params.item)) {
			const real = Reeler_vals[Reeler_keys.indexOf(params.item)];
			const aha = await fetch(`${apiUrl}/user/items/get?tipus=${real[0]}`, {
				headers: {
					cookie: cookies.get('auth_token') as string
				}
			});
			if (aha.status === 401) {
				throw redirect(302, 'noaccess');
			}

			if (aha.ok) {
				try {
					return {
						potlekok: await aha.json(),
						type: params.item,
						real,
						page: url.searchParams.get('page') ? url.searchParams.get('page') : 0
					};
				} catch {
					return {
						potlekok: undefined
					};
				}
			} else {
				return {
					error: real[1] + ' lekérése sikertelen: ' + aha.status + ' ' + aha.statusText
				};
			}
		} else {
			return {
				error: 'Nincs ilyen elem'
			};
		}
	} catch (err) {
		if ((err as Redirect).status) {
			throw redirect((err as Redirect).status, (err as Redirect).location);
		}
		return {
			error: 'Pótlékaid lekérése sikertelen'
		};
	}
}) satisfies PageServerLoad;
