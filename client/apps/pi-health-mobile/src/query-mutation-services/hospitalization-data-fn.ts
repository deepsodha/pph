import {
	type SlimJSONRPCRes,
	addHospitalizationHistory,
	deleteHospitalizationHistory,
	updateHospitalizationHistory,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	HospitalizationAdd,
	HospitalizationUpdate,
	HospitalizationDelete,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddHospitalizationDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	HospitalizationAdd,
	unknown
> {
	return MutationReuse(
		["AddHospitalization"],
		addHospitalizationHistory,
		"Add Hospitalization",
		true,
	);
}

export function UpdateHospitalizationDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	HospitalizationUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateHospitalization"],
		updateHospitalizationHistory,
		"Hospitalization Update",
		true,
	);
}

export function DeleteHospitalizationDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	HospitalizationDelete,
	unknown
> {
	return MutationReuse(
		["DeleteHospitalization"],
		deleteHospitalizationHistory,
		"Delete Hospitalization",
	);
}
