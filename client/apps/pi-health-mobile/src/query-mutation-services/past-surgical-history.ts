import {
	addPastSurgicalHistory,
	updatePastSurgicalHistory,
	deletePastSurgicalHistory,
	type SlimJSONRPCRes,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	PastSurgicalHistoryAdd,
	PastSurgicalHistoryUpdate,
	PastSurgicalHistoryDelete,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddPastSurgicalHistoryFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PastSurgicalHistoryAdd,
	unknown
> {
	return MutationReuse(
		["PastSurgicalHistoryAdd"],
		addPastSurgicalHistory,
		"PastSurgicalHistory Add",
		true,
	);
}

export function UpdatePastSurgicalHistoryFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PastSurgicalHistoryUpdate,
	unknown
> {
	return MutationReuse(
		["PastSurgicalHistoryUpdate"],
		updatePastSurgicalHistory,
		"PastSurgicalHistory Update",
		true,
	);
}

export function DeletePastSurgicalHistoryFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PastSurgicalHistoryDelete,
	unknown
> {
	return MutationReuse(
		["PastSurgicalHistoryDelete"],
		deletePastSurgicalHistory,
		"PastSurgicalHistory Delete",
	);
}
