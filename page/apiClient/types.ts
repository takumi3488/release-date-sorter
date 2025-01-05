export type SeriesDetail = {
	id: string;
	title: string;
	volumes: {
		id: string;
		title: string;
		publication_date: string;
		checked?: boolean;
	}[];
};
