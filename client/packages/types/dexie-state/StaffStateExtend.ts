import { parseISO } from "date-fns";
import type { StaffState } from "../dto";

export type StaffStateExtend = Omit<StaffState, "last_updated"> & {
	last_updated: Date;
};

export const mapStaff = (StaffStateExtend: StaffState): StaffStateExtend => {
	return {
		...StaffStateExtend,
		last_updated: parseISO(StaffStateExtend.last_updated),
	};
};
