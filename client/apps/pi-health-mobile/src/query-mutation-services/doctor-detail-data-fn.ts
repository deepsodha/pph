import {
	type SlimJSONRPCRes,
	addDoctorDetails,
	deleteDoctorDetails,
	updateDoctorDetails,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	DoctorAdd,
	DoctorDelete,
	DoctorUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddDoctorDetailDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	DoctorAdd,
	unknown
> {
	return MutationReuse(
		["AddDoctorDetail"],
		addDoctorDetails,
		"Doctor Details",
		true,
	);
}

export function UpdateDoctorDetailsDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	DoctorUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateDoctorDetail"],
		updateDoctorDetails,
		"Doctor Update",
		true,
	);
}

export function deleteDocterDetailsFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	DoctorDelete,
	unknown
> {
	return MutationReuse(
		["DeleteDoctorDetails"],
		deleteDoctorDetails,
		"Delete Doctor Details",
	);
}
