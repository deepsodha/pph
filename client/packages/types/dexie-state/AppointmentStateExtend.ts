import { parseISO } from "date-fns";
import type { AppointmentState } from "../dto";

export type AppointmentStateExtend = Omit<AppointmentState, "last_updated"> & {
	last_updated: Date;
};

export const mapAppointment = (
	AppointmentStateExtend: AppointmentState,
): AppointmentStateExtend => {
	return {
		...AppointmentStateExtend,
		last_updated: parseISO(AppointmentStateExtend.last_updated),
	};
};
