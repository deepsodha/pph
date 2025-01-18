import {
	type SlimJSONRPCRes,
	addVital,
	deleteVital,
	updateVital,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	VitalsAdd,
	VitalsDelete,
	VitalsUpdate,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddVitalsDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	VitalsAdd,
	unknown
> {
	return MutationReuse(["AddVital"], addVital, "Vital", true);
}

export function UpdateVitalsFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	VitalsUpdate,
	unknown
> {
	return MutationReuse(["UpdateVitals"], updateVital, "Vitals Updated", true);
}

export function DeleteVitalsDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	VitalsDelete,
	unknown
> {
	return MutationReuse(["DeleteVital"], deleteVital, "Delete Vital");
}
