import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async (event) => {
	const ip_address = event.getClientAddress();
	return new Response(ip_address);
};
