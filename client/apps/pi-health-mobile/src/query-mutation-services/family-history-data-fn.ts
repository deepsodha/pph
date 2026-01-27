import {
	type SlimJSONRPCRes,
	addFamilyHistory,
	deleteFamilyHistory,
	updateFamilyHistory,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	FamilyHistoryAdd,
	FamilyHistoryDelete,
	FamilyHistoryUpdate,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddFamilyHistoryDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	FamilyHistoryAdd,
	unknown
> {
	return MutationReuse(
		["FamilyHistoryAdd"],
		addFamilyHistory,
		"Family History Add",
		true,
	);
}

export function UpdateFamilyHistoryDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	FamilyHistoryUpdate,
	unknown
> {
	return MutationReuse(
		["FamilyHistoryUpdate"],
		updateFamilyHistory,
		"Family History Update",
		true,
	);
}

export function DeleteFamilyHistoryDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	FamilyHistoryDelete,
	unknown
> {
	return MutationReuse(
		["FamilyHistoryDelete"],
		deleteFamilyHistory,
		"Family History Delete",
	);
}
