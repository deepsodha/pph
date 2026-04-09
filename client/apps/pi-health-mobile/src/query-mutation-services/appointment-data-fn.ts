import {
	type SlimJSONRPCRes,
	addAppointment,
	deleteAppointment,
	updateAppointment,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	AppointmentAdd,
	AppointmentUpdate,
	AppointmentDelete,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddAppointmentDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AppointmentAdd,
	unknown
> {
	return MutationReuse(["AddAppointment"], addAppointment, "Appointment", true);
}

export function UpdateAppointmentFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AppointmentUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateAppointment"],
		updateAppointment,
		"Appointment Updated",
		true,
	);
}

export function deleteAppointmentFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AppointmentDelete,
	unknown
> {
	return MutationReuse(
		["DeleteAppointment"],
		deleteAppointment,
		"Delete Appointment",
	);
}
