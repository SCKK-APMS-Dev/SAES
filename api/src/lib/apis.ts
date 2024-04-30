import 'dotenv/config';

export function getApiUrl(api: 'erik' | 'patrik') {
	if (api === 'erik') {
		return 'https://app.sckk.hu';
	}
	if (api === 'patrik') {
		return process.env.NODE_DEV
			? 'http://api.scms.hanrickio.com:5002'
			: 'http://192.168.100.148:5002';
	}
}
