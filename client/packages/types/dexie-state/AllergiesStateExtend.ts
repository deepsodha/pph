import { parseISO } from "date-fns";
import type { AllergiesState } from "../dto";

export type AllergiesStateExtend = Omit<AllergiesState, "last_updated"> & {
	last_updated: Date;
};

export const mapAllergy = (
	AllergiesStateExtend: AllergiesState,
): AllergiesStateExtend => {
	return {
		...AllergiesStateExtend,
		last_updated: parseISO(AllergiesStateExtend.last_updated),
	};
};
