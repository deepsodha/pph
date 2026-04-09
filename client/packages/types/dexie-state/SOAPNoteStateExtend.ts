import { parseISO } from "date-fns";
import type { SOAPNoteState } from "../dto";

export type SOAPNoteStateExtend = Omit<SOAPNoteState, "last_updated"> & {
	last_updated: Date;
};

export const mapSOAPNote = (
	SOAPNoteStateExtend: SOAPNoteState,
): SOAPNoteStateExtend => {
	return {
		...SOAPNoteStateExtend,
		last_updated: parseISO(SOAPNoteStateExtend.last_updated),
	};
};
