import {
	type SlimJSONRPCRes,
	addMedication,
	deleteMedication,
	updateMedication,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type { MedicationsAdd, SyncData } from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddMedicationDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	MedicationsAdd,
	unknown
> {
	return MutationReuse(
		["AddMedication"],
		addMedication,
		"Medication Added",
		true,
	);
}

export function UpdateMedicationDataFn() {
	return MutationReuse(
		["UpdateMedication"],
		updateMedication,
		"Medication Update",
		true,
	);
}

export function DeleteMedicationDataFn() {
	return MutationReuse(
		["DeleteMedication"],
		deleteMedication,
		"Delete Medication",
	);
}
