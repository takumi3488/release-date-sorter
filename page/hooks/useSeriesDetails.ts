import useSWR from "swr";
import fetcher from "../apiClient/fetcher";
import type { SeriesDetail } from "../apiClient/types";

export function useSeriesDetails(id: string, userId?: string) {
	const url = userId ? `/api/${userId}/series/${id}` : `/api/series/${id}`;
	return useSWR<SeriesDetail>(url, fetcher);
}
