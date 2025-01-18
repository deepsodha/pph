import { parseISO } from "date-fns";
import type { PastMedicalHistoryState } from "../dto";

export type PastMedicalHistoryStateExtend = Omit<
	PastMedicalHistoryState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapPastMedicalHistory = (
	PastMedicalHistoryStateExtend: PastMedicalHistoryState,
): PastMedicalHistoryStateExtend => {
	return {
		...PastMedicalHistoryStateExtend,
		last_updated: parseISO(PastMedicalHistoryStateExtend.last_updated),
	};
};
