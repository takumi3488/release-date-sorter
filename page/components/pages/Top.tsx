import { Box, Typography } from "@mui/material";
import useSWR from "swr";
import { Link } from "wouter";
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
		<Box>
			<ul>
				{data.map((series) => (
					<li key={series.id} style={{ marginBottom: "0.5rem" }}>
						<Link href={`/series/${series.id}`}>
							<Typography>{series.title}</Typography>
						</Link>
					</li>
				))}
			</ul>
		</Box>
	);
}
