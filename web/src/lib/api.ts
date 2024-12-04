export const apiUrl = process.env.NODE_ENV === "development"
	? "http://localhost:3000"
	: (process.env.api_prod_url as string);

export const snow = false;
