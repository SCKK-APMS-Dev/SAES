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
	}
}
