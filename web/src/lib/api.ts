export const apiUrl =
	process.env.NODE_ENV === 'development' ? 'http://localhost:3000' : process.env.API_PROD_URL;
