import { parseISO } from "date-fns";
import type { MedicationsState } from "../dto";

export type MedicationsStateExtend = Omit<MedicationsState, "last_updated"> & {
	last_updated: Date;
};

export const mapMedication = (
	MedicationsStateExtend: MedicationsState,
): MedicationsStateExtend => {
	return {
		...MedicationsStateExtend,
		last_updated: parseISO(MedicationsStateExtend.last_updated),
	};
};
