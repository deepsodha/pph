import {
	type SlimJSONRPCRes,
	addStaffDetails,
	updateStaffDetails,
	deleteStaffDetails,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	StaffAdd,
	StaffDelete,
	StaffUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddStaffDetailDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	StaffAdd,
	unknown
> {
	return MutationReuse(
		["AddStaffDetail"],
		addStaffDetails,
		"Staff Details",
		true,
	);
}

export function UpdateStaffDetailsDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	StaffUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateStaffDetail"],
		updateStaffDetails,
		"Staff Update",
		true,
	);
}

export function deleteStaffDetailsFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	StaffDelete,
	unknown
> {
	return MutationReuse(
		["DeleteStaffDetails"],
		deleteStaffDetails,
		"Delete Staff Details",
	);
}
