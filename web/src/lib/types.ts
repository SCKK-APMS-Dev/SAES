export interface FullDBType {
	faction: string;
	date: Date;
	price: null | number;
	id: number;
	image: string;
	owner: string;
	reason: null | string;
	status: number;
	type: number;
}

export interface SMGetItemsFull {
	id: number;
	owner: string;
	img_1: number;
	img_2: number | undefined;
	status: number;
	reason: string | undefined;
	type: number | undefined;
	price: number | undefined;
	faction: number;
	handled_by: string | undefined;
	date: Date;
}
