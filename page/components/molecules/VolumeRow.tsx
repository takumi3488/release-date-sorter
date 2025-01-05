import { TableCell, TableRow } from "@mui/material";
import VolumeCheckCell from "../atoms/VolumeCheckCell";

export default function VolumeRow({
	volume,
}: {
	volume: {
		id: string;
		title: string;
		publication_date: string;
		checked?: boolean;
	};
}) {
	return (
		<TableRow>
			{volume.checked !== undefined && (
				<VolumeCheckCell id={volume.id} checked={volume.checked} />
			)}
			<TableCell>
				{volume.publication_date.replace("-", "年").replace("-", "月")}日
			</TableCell>
			<TableCell>{volume.title}</TableCell>
		</TableRow>
	);
}
