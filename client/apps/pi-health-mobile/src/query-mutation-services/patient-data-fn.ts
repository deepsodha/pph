import {
	type SlimJSONRPCRes,
	addPatient,
	deletePatient,
	updatePatient,
} from "@repo/services/src";
import type {
	PatientAdd,
	PatientDelete,
	PatientUpdate,
	SyncData,
} from "@repo/types/dto";
import { MutationReuse } from "./mutation-reuse";
import type { UseMutationResult } from "@tanstack/react-query";

export function UpdatePatientFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PatientUpdate,
	unknown
> {
	return MutationReuse(
		["UpdatePatient"],
		updatePatient,
		"Patient Updated",
		true,
	);
}

export function AddPatientDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PatientAdd,
	unknown
> {
	return MutationReuse<PatientAdd>(["AddPatient"], addPatient, "patient", true);
}

export function deletePatientFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	PatientDelete,
	unknown
> {
	return MutationReuse<PatientDelete>(
		["DeletePatient"],
		deletePatient,
		"Delete Patient",
	);
}
