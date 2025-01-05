import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableRow,
} from "@mui/material";
import VolumeRow from "../molecules/VolumeRow";

export default function VolumeTable({
	volumes,
	userId,
}: {
	volumes: {
		id: string;
		title: string;
		publication_date: string;
		checked?: boolean;
	}[];
	userId?: string;
}) {
	return (
		<Table size="small">
			<TableHead>
				<TableRow sx={{ backgroundColor: "#f5f5f5" }}>
					{userId && <TableCell />}
					<TableCell>発売日</TableCell>
					<TableCell>タイトル</TableCell>
				</TableRow>
			</TableHead>
			<TableBody>
				{volumes.map((volume) => (
					<VolumeRow key={volume.id} volume={volume} />
				))}
			</TableBody>
		</Table>
	);
}
