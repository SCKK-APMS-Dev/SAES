import { type Redirect, redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import { apiUrl } from "$lib/api";

export const load = (async ({ cookies, url }) => {
	try {
		const aha = await fetch(`${apiUrl}/ucp/mv`, {
			mode: "no-cors",
			headers: {
				cookie: cookies.get("auth_token") as string,
			},
		});
		if (aha.status === 404) {
			throw redirect(
				302,
				`${apiUrl}/auth?path=${url.pathname}`,
			);
		}
		if (aha.status === 403) {
			throw redirect(302, "/ucp");
		}

		if (aha.ok) {
			return {};
		}
	} catch (err) {
		if ((err as Redirect).status) {
			throw redirect(
				(err as Redirect).status,
				(err as Redirect).location,
			);
		}
		return {
			error: true,
		};
	}
}) satisfies LayoutServerLoad;
