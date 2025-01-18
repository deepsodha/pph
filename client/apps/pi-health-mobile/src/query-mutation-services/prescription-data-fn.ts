import {
	type SlimJSONRPCRes,
	addPrescription,
	deletePrescription,
	updatePrescription,
} from "@repo/services/src";
import type {
	PrescriptionAdd,
	PrescriptionDelete,
	PrescriptionUpdate,
	SyncData,
} from "@repo/types/dto";
import { MutationReuse } from "./mutation-reuse";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddPrescriptionDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PrescriptionAdd,
	unknown
> {
	return MutationReuse(
		["AddPrescription"],
		addPrescription,
		"Prescription",
		true,
	);
}

export function deletePrescriptionFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PrescriptionDelete,
	unknown
> {
	return MutationReuse(
		["Delete Prescription"],
		deletePrescription,
		"Delete Prescription",
	);
}

export function UpdatePrescriptionFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PrescriptionUpdate,
	unknown
> {
	return MutationReuse(
		["UpdatePrescription"],
		updatePrescription,
		"Prescription Updated",
		true,
	);
}
