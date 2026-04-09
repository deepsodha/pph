import { parseISO } from "date-fns";
import type { HistoryAndPhysicalNoteState } from "../dto";

export type HistoryAndPhysicalNoteStateExtend = Omit<
	HistoryAndPhysicalNoteState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapHistoryAndPhysicalNote = (
	HistoryAndPhysicalNoteStateExtend: HistoryAndPhysicalNoteState,
): HistoryAndPhysicalNoteStateExtend => {
	return {
		...HistoryAndPhysicalNoteStateExtend,
		last_updated: parseISO(HistoryAndPhysicalNoteStateExtend.last_updated),
	};
};
