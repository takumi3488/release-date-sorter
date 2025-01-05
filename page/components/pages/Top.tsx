import useSWR from "swr";
import fetcher from "../../apiClient/fetcher";

export default function Top() {
	type Series = {
		id: string;
		title: string;
	};
	const { data, error } = useSWR<Series[]>("/api/series", fetcher);
	if (error) return <div>failed to load</div>;
	if (!data) return <div>loading...</div>;
	return (
		<div>
			{data.map((series) => (
				<p key={series.id}>{series.id}</p>
			))}
		</div>
	);
}
