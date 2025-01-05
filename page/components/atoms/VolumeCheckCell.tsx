import { Checkbox, TableCell } from "@mui/material";
import { mutate } from "swr";
import { useRoute } from "wouter";
import type { SeriesDetail } from "../../apiClient/types";

export default function VolumeCheckCell({
	id,
	checked,
}: {
	id: string;
	checked: boolean;
}) {
	const [match, params] = useRoute("/series/:id/:userId");
	async function hanndleCheck() {
		if (!match) return;
		async function check(userId: string, volumeId: string) {
			if (!match) return;
			await fetch("/api/user_volumes", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					volume_id: volumeId,
					user_id: userId,
					checked: !checked,
				}),
			});
			return await fetch(`/api/${userId}/series/${params.id}`).then((res) =>
				res.json(),
			);
		}
		await mutate(
			`/api/${params.userId}/series/${params.id}`,
			check(params.userId, id),
			{
				optimisticData: updateData,
			},
		);
	}
	function updateData(data?: SeriesDetail): SeriesDetail {
		if (!match || !data)
			return {
				id: "",
				title: "",
				volumes: [],
			};
		if (data.id === params.id) {
			return {
				...data,
				volumes: data.volumes.map((volume) => {
					if (volume.id === id) {
						return {
							...volume,
							checked: !checked,
						};
					}
					return volume;
				}),
			};
		}
		return data;
	}

	return (
		<TableCell>
			<Checkbox checked={checked} onClick={hanndleCheck} />
		</TableCell>
	);
}
