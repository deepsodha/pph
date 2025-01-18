import {
	addImplantableDevices,
	updateImplantableDevices,
	deleteImplantableDevices,
	type SlimJSONRPCRes,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	ImplantableDevicesAdd,
	ImplantableDevicesDelete,
	ImplantableDevicesUpdate,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddImplantableDevicesDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ImplantableDevicesAdd,
	unknown
> {
	return MutationReuse(
		["ImplantableDevicesAdd"],
		addImplantableDevices,
		"ImplantableDevices Add",
		true,
	);
}

export function UpdateImplantableDevicesDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ImplantableDevicesUpdate,
	unknown
> {
	return MutationReuse(
		["ImplantableDevicesUpdate"],
		updateImplantableDevices,
		"ImplantableDevices Update",
		true,
	);
}

export function DeleteImplantableDevicesDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ImplantableDevicesDelete,
	unknown
> {
	return MutationReuse(
		["ImplantableDevicesDelete"],
		deleteImplantableDevices,
		"ImplantableDevices Delete",
	);
}
