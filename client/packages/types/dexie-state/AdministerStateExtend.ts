import { parseISO } from "date-fns";
import type { AdministerState } from "../dto";

export type AdministerStateExtend = Omit<AdministerState, "last_updated"> & {
	last_updated: Date;
};

export const mapAdminister = (
	AdministerStateExtend: AdministerState,
): AdministerStateExtend => {
	return {
		...AdministerStateExtend,
		last_updated: parseISO(AdministerStateExtend.last_updated),
	};
};
