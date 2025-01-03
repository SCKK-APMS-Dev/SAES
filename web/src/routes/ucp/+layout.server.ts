import type { LayoutServerLoad } from "./$types";
import { apiUrl, imageUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";

export const load = (async ({ cookies, request, url }) => {
	if (!cookies.get("auth_token")) {
		return {
			noauth: true,
			apiUrl,
		};
	}
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
		if (url.searchParams.get("select_faction")) {
			let sfact = url.searchParams.get("select_faction") as string;
			if (["SCKK", "TOW"].includes(sfact)) {
				if (sfact === "SCKK" && (jeson.taxi || jeson.admin)) {
					cookies.set("selected_faction", "SCKK", {
						path: "/",
						maxAge: 60 * 60 * 24 * 7,
						secure: true,
						sameSite: true,
						httpOnly: true,
					});
					throw redirect(303, url.pathname);
				}
				if (sfact === "TOW" && (jeson.tow || jeson.admin)) {
					cookies.set("selected_faction", "TOW", {
						path: "/",
						maxAge: 60 * 60 * 24 * 7,
						secure: true,
						sameSite: true,
						httpOnly: true,
					});
					throw redirect(303, url.pathname);
				}
			}
		}
		if (!cookies.get("selected_faction")) {
			return {
				layout: jeson,
				api: apiUrl,
				auth: cookies.get("auth_token")!,
				maintenance: cookies.get("maintenance")
					? jeson.admin ? cookies.get("maintenance") : false
					: false,
				nofact: true,
			};
		}
		return {
			layout: jeson,
			api: apiUrl,
			image: imageUrl,
			faction: cookies.get("selected_faction"),
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
}) satisfies LayoutServerLoad;
