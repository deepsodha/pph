import {
	type SlimJSONRPCRes,
	addSystemAdmin,
	deleteSystemAdmin,
	updateSystemAdmin,
	getSystemAdmin,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	GetSystemadmin,
	SyncData,
	SystemAdminAdd,
	SystemAdminDelete,
	SystemAdminUpdate,
} from "@repo/types/dto";
import {
	useQuery,
	type UseMutationResult,
	type UseQueryResult,
} from "@tanstack/react-query";

export function AddSystemAdminDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	SystemAdminAdd,
	unknown
> {
	return MutationReuse(
		["AddSystemAdmin"],
		addSystemAdmin,
		"System Admin",
		true,
	);
}
export function GetSystemAdminDataFn(): UseQueryResult<
	SlimJSONRPCRes<GetSystemadmin[], unknown>,
	Error
> {
	return useQuery({
		queryKey: ["GetSystemAdmin"],
		queryFn: () => getSystemAdmin(),
	});
}

export function UpdateSystemAdminFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	SystemAdminUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateSystemAdmin"],
		updateSystemAdmin,
		"System Admin Updated",
		true,
	);
}

export function DeleteSystemAdminDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	SystemAdminDelete,
	unknown
> {
	return MutationReuse(
		["DeleteSystemAdmin"],
		deleteSystemAdmin,
		"Delete System Admin",
	);
}
