// import { sheet } from '$lib/server/google';
// import { prisma } from '$lib/server/prisma';
import type { PageServerLoad } from "./$types";
import { apiUrl } from "$lib/api";

export const load = (async ({ cookies }) => {
	if (!cookies.get("auth_token")) {
		return {
			noauth: true,
			apiUrl,
		};
	}
	try {
		const aha = await fetch(`${apiUrl}/ucp/calls`, {
			mode: "no-cors",
			headers: {
				cookie: cookies.get("auth_token") as string,
			},
		});
		if (aha.status === 400) {
			return {
				error: "Központi API szerver elérése sikertelen",
			};
		}
		if (aha.ok) {
			const text = await aha.json();
			return {
				calls: text,
			};
		}
	} catch {
		return {
			error: "Weboldal API szervere elérése sikertelen",
		};
	}
}) satisfies PageServerLoad;
