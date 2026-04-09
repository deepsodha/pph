import {
	type SlimJSONRPCRes,
	addAdminister,
	deleteAdminister,
	updateAdminister,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	AdministerAdd,
	AdministerDelete,
	AdministerUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddAdministerDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AdministerAdd,
	unknown
> {
	return MutationReuse(
		["AdministerAdd"],
		addAdminister,
		"Administer Add",
		true,
	);
}

export function UpdateAdministerDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AdministerUpdate,
	unknown
> {
	return MutationReuse<AdministerUpdate>(
		["AdministerUpdate"],
		updateAdminister,
		"Administer Update",
		true,
	);
}

export function DeleteAdministerDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AdministerDelete,
	unknown
> {
	return MutationReuse<AdministerDelete>(
		["AdministerDelete"],
		deleteAdminister,
		"Administer Delete",
	);
}
