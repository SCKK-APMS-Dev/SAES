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
	if (aha.status === 402) {
		return {
			error: await aha.text(),
		};
	}
	if (aha.ok) {
		const jeson: {
			discordid: string;
			driverid: number;
			name: string;
			admin: boolean;
			perms: string[];
			taxi?: {
				factionid: number;
				factionname: string;
				positionid: number;
				positionname: string;
				shiftid: number;
				shiftname: string;
			};
			tow?: {
				factionid: number;
				factionname: string;
				positionid: number;
				positionname: string;
				shiftid: number;
				shiftname: string;
			};
		} = await aha.json();
		if (jeson.name) {
			if (url.searchParams.get("select_faction")) {
				let sfact = url.searchParams.get("select_faction") as string;
				if (["SCKK", "TOW"].includes(sfact)) {
					if (
						sfact === "SCKK" &&
						(jeson.perms.includes("saes.ucp.taxi") || jeson.admin)
					) {
						cookies.set("selected_faction", "SCKK", {
							path: "/",
							maxAge: 60 * 60 * 24 * 7,
							secure: true,
							sameSite: true,
							httpOnly: true,
						});
						throw redirect(303, url.pathname);
					}
					if (
						sfact === "TOW" &&
						(jeson.perms.includes("saes.ucp.tow") || jeson.admin)
					) {
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
			if (url.searchParams.get("clear_faction")) {
				cookies.delete("selected_faction", { path: "/" });
				throw redirect(303, url.pathname);
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
			switch (cookies.get("selected_faction")) {
				case "SCKK":
					if (
						!jeson.perms.includes("saes.ucp.taxi") && !jeson.admin
					) {
						console.log("nincs");
						throw redirect(303, "?clear_faction=true");
					}
					break;
				case "TOW":
					if (!jeson.perms.includes("saes.ucp.tow") && !jeson.admin) {
						throw redirect(303, "?clear_faction=true");
					}
					break;
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
		} else {
			return {
				noaccess: "SAMT API elérése sikertelen.",
			};
		}
	}
}) satisfies LayoutServerLoad;
