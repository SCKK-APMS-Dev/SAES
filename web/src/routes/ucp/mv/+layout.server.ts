import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import { apiUrl } from "$lib/api";

export const load = (async ({ cookies }) => {
	if (!cookies.get("auth_token")) {
		return {
			noauth: true,
			apiUrl,
		};
	}
	try {
		const aha = await fetch(`${apiUrl}/ucp/mv`, {
			mode: "no-cors",
			headers: {
				cookie: cookies.get("auth_token") as string,
			},
		});
		if (aha.status === 404) {
			return {
				noauth: true,
				apiUrl,
			};
		}
		if (aha.status === 403) {
			throw redirect(302, "/ucp");
		}

		if (aha.ok) {
			return {
				success: true,
			};
		}
	} catch (err) {
		return {
			error: true,
		};
	}
}) satisfies LayoutServerLoad;
