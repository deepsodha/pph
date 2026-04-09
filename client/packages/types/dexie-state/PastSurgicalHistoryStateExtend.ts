import { parseISO } from "date-fns";
import type { PastSurgicalHistoryState } from "../dto";

export type PastSurgicalHistoryStateExtend = Omit<
	PastSurgicalHistoryState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapPastSurgicalHistory = (
	PastSurgicalHistoryStateExtend: PastSurgicalHistoryState,
): PastSurgicalHistoryStateExtend => {
	return {
		...PastSurgicalHistoryStateExtend,
		last_updated: parseISO(PastSurgicalHistoryStateExtend.last_updated),
	};
};
