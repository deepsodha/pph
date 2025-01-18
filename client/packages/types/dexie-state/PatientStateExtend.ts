import { parseISO } from "date-fns";
import type { PatientState } from "../dto/PatientState";

export type PatientStateExtend = Omit<PatientState, "last_updated"> & {
	last_updated: Date;
};

export const mapPatient = (PatientExtend: PatientState): PatientStateExtend => {
	return {
		...PatientExtend,
		last_updated: parseISO(PatientExtend.last_updated),
	};
};
