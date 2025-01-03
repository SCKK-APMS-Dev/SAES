import type { LayoutServerLoad } from "./$types";
import { apiUrl, imageUrl } from "$lib/api";

export const load = (async ({ cookies, request }) => {
	if (!cookies.get("auth_token")) {
		return {
			noauth: true,
			apiUrl,
		};
	}
	try {
		const aha = await fetch(`${apiUrl}/ucp`, {
			headers: {
				cookie: cookies.get("auth_token")!,
			},
		});
		if (aha.status === 404 || aha.status === 406) {
			return {
				noauth: true,
				apiUrl,
			};
		}
		if (aha.status === 403) {
			return {
				noaccess: await aha.text(),
			};
		}
		if (aha.ok) {
			const jeson = await aha.json();
			return {
				layout: jeson,
				api: apiUrl,
				image: imageUrl,
				country: process.env.NODE_ENV === "development"
					? "HU"
					: (request.headers.get("cf-ipcountry") as string),
				auth: cookies.get("auth_token")!,
				offset: process.env.SUMMER_TIMEZONE === "true"
					? -60 * 60 * 1000 * 2
					: -60 * 60 * 1000,
				agent: request.headers.get("user-agent") as string,
				maintenance: cookies.get("maintenance")
					? jeson.admin ? cookies.get("maintenance") : false
					: false,
			};
		}
	} catch (err) {
		return {
			error: "Weboldal API szerverét nem sikerült elérni",
		};
	}
}) satisfies LayoutServerLoad;
