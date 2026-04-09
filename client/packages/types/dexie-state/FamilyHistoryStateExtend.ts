import { parseISO } from "date-fns";
import type { FamilyHistoryState } from "../dto";

export type FamilyHistoryStateExtend = Omit<
	FamilyHistoryState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapFamilyHistory = (
	FamilyHistoryStateExtend: FamilyHistoryState,
): FamilyHistoryStateExtend => {
	return {
		...FamilyHistoryStateExtend,
		last_updated: parseISO(FamilyHistoryStateExtend.last_updated),
	};
};
