import {
	type SlimJSONRPCRes,
	addUser,
	updateUser,
	deleteUser,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	UserAdd,
	UserDelete,
	UserUpdate,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddUserDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	UserAdd,
	unknown
> {
	return MutationReuse(["AddUser"], addUser, "User", true);
}

export function UpdateUserDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	UserUpdate,
	unknown
> {
	return MutationReuse(["UpdateUser"], updateUser, "User Update", true);
}

export function deleteUserFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	UserDelete,
	unknown
> {
	return MutationReuse(["DeleteUsers"], deleteUser, "Delete User");
}
