import { type Redirect, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { apiUrl } from "$lib/api";

export const load = (async ({ cookies, url }) => {
	try {
		const aha = await fetch(`${apiUrl}/ucp/items/get?tipus=potlek`, {
			headers: {
				cookie: cookies.get("auth_token") as string,
			},
		});
		if (aha.status === 401) {
			throw redirect(302, "noaccess");
		}

		if (aha.ok) {
			try {
				return {
					potlekok: await aha.json(),
					offset: new Date().getTimezoneOffset() * 60 * 1000,
					page: url.searchParams.get("page")
						? url.searchParams.get("page")
						: 0,
				};
			} catch {
				return {
					potlekok: undefined,
				};
			}
		} else {
			return {
				error: "Pótlék lekérése sikertelen: " + aha.status + " " +
					aha.statusText,
			};
		}
	} catch (err) {
		if ((err as Redirect).status) {
			throw redirect(
				(err as Redirect).status,
				(err as Redirect).location,
			);
		}
		return {
			error: "Pótlékaid lekérése sikertelen",
		};
	}
}) satisfies PageServerLoad;
