import { Alert, Box, Typography } from "@mui/material";
import { Link } from "wouter";
import { useSeriesDetails } from "../../hooks/useSeriesDetails";
import VolumeTable from "../organisms/VolumeTable";

export default function Series({
	id,
	userId,
}: { id: string; userId?: string }) {
	const { data: series, error } = useSeriesDetails(id, userId);
	if (error) return <div>failed to load</div>;
	if (!series) return <div>loading...</div>;
	return (
		<Box
			sx={{
				gap: 2,
				display: "flex",
				flexDirection: "column",
				maxWidth: "60rem",
			}}
		>
			<Typography variant="h4" component="h2">
				{series.title}
			</Typography>
			{userId ? (
				<Alert severity="info">
					このページをブックマークすることで進捗を保存できます。
					このURLは他の人と共有しないでください。
				</Alert>
			) : (
				<Link href={`/series/${id}/${crypto.randomUUID()}`}>
					URLを発行して進捗を保存する
				</Link>
			)}
			<VolumeTable volumes={series.volumes} userId={userId} />
		</Box>
	);
}
