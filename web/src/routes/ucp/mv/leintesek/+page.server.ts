import type { PageServerLoad } from "./$types";

export const load = (async ({ url }) => {
	return {
		page: url.searchParams.get("page") ? url.searchParams.get("page") : 0,
		offset: new Date().getTimezoneOffset() * 60 * 1000,
		status: url.searchParams.get("status")
			? url.searchParams.get("status")
			: "feltöltve",
	};
}) satisfies PageServerLoad;
