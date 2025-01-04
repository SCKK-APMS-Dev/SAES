import { apiUrl } from "$lib/api";
import type { FullDBType } from "$lib/types";
import type { PageServerLoad } from "./$types";

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

export const load = (async ({ cookies, params }) => {
	const weektypes = ["current", "previous"];
	if (!weektypes.includes(params.week)) {
		return {
			error: "Ilyen hét nincs!",
		};
	}
	const dcauth = cookies.get("auth_token");
	if (dcauth) {
		const mama = await fetch(`${apiUrl}/ucp/sm/stat?week=${params.week}`, {
			headers: {
				cookie: dcauth,
				faction: cookies.get("selected_faction")!,
			},
		});
		if (mama.ok) {
			const ret: returnstat = await mama.json();
			return {
				stats: ret.stats,
				week: params.week,
				date: {
					next: ret.date.next,
					prev: ret.date.prev,
				},
			};
		}
	}
}) satisfies PageServerLoad;
