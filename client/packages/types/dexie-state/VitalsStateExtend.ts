import { parseISO } from "date-fns";
import type { VitalsState } from "../dto";

export type VitalsStateExtend = Omit<VitalsState, "last_updated"> & {
	last_updated: Date;
};

export const mapVital = (VitalsStateExtend: VitalsState): VitalsStateExtend => {
	return {
		...VitalsStateExtend,
		last_updated: parseISO(VitalsStateExtend.last_updated),
	};
};
