export const POST = async ({ request }) => {
	// const body = await request.text();
	return new Response(request.headers.get('cookies'));
};
