import { parseISO } from "date-fns";
import type { PrescriptionState } from "../dto";

export type PrescriptionStateExtend = Omit<
	PrescriptionState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapPrescription = (
	PrescriptionStateExtend: PrescriptionState,
): PrescriptionStateExtend => {
	return {
		...PrescriptionStateExtend,
		last_updated: parseISO(PrescriptionStateExtend.last_updated),
	};
};
