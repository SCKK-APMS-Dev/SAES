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
const Reeler = {
	leintes: ['leintés', 'Leintéseid', 'Leintés', 'Leintések', 'leintésének'],
	potlek: ['pótlék', 'Pótlékaid', 'Pótlék', 'Pótlékok', 'pótlékjának'],
	szamla: ['számla', 'Számláid', 'Számla', 'Számlák', 'számlájának']
};

export const Reeler_keys = Object.keys(Reeler);
export const Reeler_vals = Object.values(Reeler);
