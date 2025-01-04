import { apiUrl, imageUrl } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load = (async ({ params }) => {
	const aha = await fetch(
		`${apiUrl}/list?driver=${
			params.ppl.replace("_", " ")
		}&tipus=${params.type}`,
	);

	if (aha.ok) {
		const text = await aha.json();
		return {
			cucc: text,
			type: params.type,
			api: apiUrl,
			image: imageUrl,
		};
	}
}) satisfies PageServerLoad;
