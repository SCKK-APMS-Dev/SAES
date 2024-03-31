import 'dotenv/config';

export function getApiUrl(api: 'erik' | 'patrik') {
	if (api === 'erik') {
		return process.env.NODE_DEV ? 'https://app.sckk.hu' : 'http://192.168.100.138:5472';
	}
	if (api === 'patrik') {
		return process.env.NODE_DEV
			? 'http://api.scms.hanrickio.com:5002'
			: 'http://192.168.100.148:5002';
	}
}
