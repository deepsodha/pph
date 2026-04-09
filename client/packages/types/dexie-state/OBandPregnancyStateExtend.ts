import { parseISO } from "date-fns";
import type { OBandPregnancyState } from "../dto";

export type OBandPregnancyStateExtend = Omit<
	OBandPregnancyState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapOBandPregnancy = (
	OBandPregnancyStateExtend: OBandPregnancyState,
): OBandPregnancyStateExtend => {
	return {
		...OBandPregnancyStateExtend,
		last_updated: parseISO(OBandPregnancyStateExtend.last_updated),
	};
};
