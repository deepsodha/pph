import { parseISO } from "date-fns";
import type { HistoricalState } from "../dto";

export type HistoricalStateExtend = Omit<HistoricalState, "last_updated"> & {
	last_updated: Date;
};

export const mapHistorical = (
	HistoricalStateExtend: HistoricalState,
): HistoricalStateExtend => {
	return {
		...HistoricalStateExtend,
		last_updated: parseISO(HistoricalStateExtend.last_updated),
	};
};
