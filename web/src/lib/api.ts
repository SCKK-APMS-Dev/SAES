export const apiUrl =
	process.env.NODE_ENV === 'development'
		? 'http://localhost:3000'
		: (process.env.api_url as string);
export const cdnUrl =
	process.env.NODE_ENV === 'development'
		? 'http://localhost:3100'
		: (process.env.cdn_url as string);

let date = new Date();

// * Hó engedélyezése Mikulás-naptól vízkeresztig
export const snow =
	(date.getMonth() === 11 && date.getDate() >= 6) || (date.getMonth() < 2 && date.getDate() < 20)
		? true
		: false;

export const christmas =
	(date.getMonth() === 11 && date.getDate() >= 6) || (date.getMonth() == 0 && date.getDate() <= 7)
		? true
		: false;
