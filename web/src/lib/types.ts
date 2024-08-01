export interface FullDBType {
	am: 0 | 1;
	date: Date;
	extra: null | string;
	id: number;
	kep: string;
	owner: string;
	reason: null | string;
	status: 'feltöltve' | 'elfogadva' | 'elutasítva';
	type: 'pótlék' | 'leintés' | 'számla';
}
