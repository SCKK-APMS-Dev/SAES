import { get_status_number } from "$lib/ucp/types";
import type { PageServerLoad } from "./$types";

export const load = (async ({ url }) => {
	return {
		page: url.searchParams.get("page") ? url.searchParams.get("page") : 0,
		status: url.searchParams.get("status")
			? url.searchParams.get("status")
			: get_status_number("feltöltve").toString(),
	};
}) satisfies PageServerLoad;
