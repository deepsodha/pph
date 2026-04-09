import { parseISO } from "date-fns";
import type { DoctorState } from "../dto";

export type DoctorStateExtend = Omit<DoctorState, "last_updated"> & {
	last_updated: Date;
};

export const mapDoctor = (
	DoctorStateExtend: DoctorState,
): DoctorStateExtend => {
	return {
		...DoctorStateExtend,
		last_updated: parseISO(DoctorStateExtend.last_updated),
	};
};
