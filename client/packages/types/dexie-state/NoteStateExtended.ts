import { parseISO } from "date-fns";
import type { NoteState } from "../dto/NoteState";

export type NoteStateExtend = Omit<NoteState, "last_updated"> & {
	last_updated: Date;
};

export const mapNote = (NoteStateExtend: NoteState): NoteStateExtend => {
	return {
		...NoteStateExtend,
		last_updated: parseISO(NoteStateExtend.last_updated),
	};
};
