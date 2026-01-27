import {
	type SlimJSONRPCRes,
	addAllergy,
	deleteAllergy,
	updateAllergy,
	getAllergyName,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	AllergyAdd,
	AllergyDelete,
	AllergyUpdate,
	SyncData,
} from "@repo/types/dto";
import { type UseMutationResult, useQuery } from "@tanstack/react-query";

export function AddAllergyDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AllergyAdd,
	unknown
> {
	return MutationReuse(["AllergyAdd"], addAllergy, "Allergy", true);
}

export function UpdateAllergyFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AllergyUpdate,
	unknown
> {
	return MutationReuse(["AllergyUpdate"], updateAllergy, "Allergy", true);
}

export function DeleteAllergyDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	AllergyDelete,
	unknown
> {
	return MutationReuse(["DeleteAllergy"], deleteAllergy, "Delete Allergy");
}

export function GetAllergyNameDataFn() {
	return useQuery({
		queryKey: ["GetOrganizationCountry"],
		queryFn: () => getAllergyName(),
	});
}
