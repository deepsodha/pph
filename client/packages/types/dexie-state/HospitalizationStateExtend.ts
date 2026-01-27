import { parseISO } from "date-fns";
import type { HospitalizationState } from "../dto";

export type HospitalizationStateExtend = Omit<
	HospitalizationState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapHospitalization = (
	HospitalizationStateExtend: HospitalizationState,
): HospitalizationStateExtend => {
	return {
		...HospitalizationStateExtend,
		last_updated: parseISO(HospitalizationStateExtend.last_updated),
	};
};
