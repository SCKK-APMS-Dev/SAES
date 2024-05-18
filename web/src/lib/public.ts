export function getRealText(text: string) {
	switch (text) {
		case 'pótlék_délelőtti':
			return 'Délelőtti pótlékok';
		case 'pótlék_éjszakai':
			return 'Éjszakai pótlékok';
		case 'számla':
			return 'Számlák';
		case 'leintés':
			return 'Leintések';
		case 'am_pótlék_délelőtti':
			return 'AM délelőtti pótlékok';
		case 'am_pótlék_éjszakai':
			return 'AM éjszakai pótlékok';
		case 'am_számla':
			return 'AM számlák';
		case 'am_leintés':
			return 'AM leintések';
	}
}
