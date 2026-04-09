import { parseISO } from "date-fns";
import type { SystemAdminState } from "../dto";

export type SystemAdminStateExtend = Omit<SystemAdminState, "last_updated"> & {
	last_updated: Date;
};

export const mapSystemAdmin = (
	SystemAdminStateExtend: SystemAdminState,
): SystemAdminStateExtend => {
	return {
		...SystemAdminStateExtend,
		last_updated: parseISO(SystemAdminStateExtend.last_updated),
	};
};
