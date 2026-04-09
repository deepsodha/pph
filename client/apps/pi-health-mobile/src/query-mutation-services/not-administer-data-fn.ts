import {
	type SlimJSONRPCRes,
	addNotAdminister,
	deleteNotAdminister,
	updateNotAdminister,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	NotAdministeredAdd,
	NotAdministeredDelete,
	NotAdministeredUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddNotAdministerDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	NotAdministeredAdd,
	unknown
> {
	return MutationReuse(
		["AddNotAdminister"],
		addNotAdminister,
		"Not Administer",
		true,
	);
}

export function UpdateNotAdministerDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	NotAdministeredUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateNotAdminister"],
		updateNotAdminister,
		"Not Administer Update",
		true,
	);
}

export function DeleteNotAdministerDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	NotAdministeredDelete,
	unknown
> {
	return MutationReuse(
		["DeleteNotAdminister"],
		deleteNotAdminister,
		"Delete Not Administer",
	);
}
