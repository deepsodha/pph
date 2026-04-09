import {
	type SlimJSONRPCRes,
	addPastMedicalHistory,
	deletePastMedicalHistory,
	updatePastMedicalHistory,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	PastMedicalHistoryAdd,
	PastMedicalHistoryUpdate,
	PastMedicalHistoryDelete,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddPastMedicalHistoryDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PastMedicalHistoryAdd,
	unknown
> {
	return MutationReuse(
		["PastMedicalHistoryAdd"],
		addPastMedicalHistory,
		"Past Medical History Add",
		true,
	);
}

export function UpdatePastMedicalHistoryDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PastMedicalHistoryUpdate,
	unknown
> {
	return MutationReuse(
		["PastMedicalHistoryUpdate"],
		updatePastMedicalHistory,
		"Past Medical History Update",
		true,
	);
}

export function DeletePastMedicalHistoryDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PastMedicalHistoryDelete,
	unknown
> {
	return MutationReuse(
		["PastMedicalHistoryDelete"],
		deletePastMedicalHistory,
		"Past Medical History Delete",
		true,
	);
}
